use crate::event::ConvEvent;
use crate::model::cfile::CFileBmc;
use crate::model::cfile_db::conv_ref::ConvRefBmc;
use crate::model::cfile_db::msg::{Msg, MsgBmc, MsgFilter, MsgForCreate};
use crate::model::stack_step::{StackStep, StackStepBmc, StackStepFilter, StackStepLite};
use crate::model::support::prelude::*;
use derive_more::From;
use lib_utils::time::now;
use modql::field::SeaField;
use tracing::error;

// region:    --- Types

#[cfg_attr(feature = "for-ts", derive(schemars::JsonSchema))]
#[skip_serializing_none]
#[derive(Debug, Clone, Fields, FromSqliteRow, Serialize, Deserialize)]
#[modql(names_as_consts)]
pub struct Conv {
	pub id: Id,
	pub uid: String,

	pub space_id: Id,

	pub cfile_id: Option<Id>,

	// Note: This is mostly for perf reason to avoid doing a lookup
	//       on the cfile_db conv_ref table when we insert Msg in the cfile_db msg table.
	//       The cfile_db conv_ref have a `conv_uid` which is this `Conv.uid` as well.
	pub conv_ref_id: Option<Id>,

	pub title: Option<String>,

	pub work_tnew: Option<UnixTimeUs>,
	pub work_tdone: Option<UnixTimeUs>,

	pub last_open: Option<i64>,
}

#[derive(Fields, Deserialize)]
pub struct ConvForCreate {
	pub space_id: Id,

	pub title: Option<String>,
}

// impl new
impl ConvForCreate {
	pub fn new(space_id: Id) -> Self {
		Self { space_id, title: None }
	}
}

#[derive(Fields, Default, Deserialize)]
pub struct ConvForUpdate {
	pub title: Option<String>,
	pub cfile_id: Option<Id>,
	pub conv_ref_id: Option<Id>,

	pub work_tnew: Option<UnixTimeUs>,
	pub work_tdone: Option<UnixTimeUs>,
}

#[derive(FilterNodes, Default, Deserialize)]
pub struct ConvFilter {
	pub uid: Option<OpValsString>,

	pub space_id: Option<OpValsInt64>,
}

// endregion: --- Types

// region:    --- ConvBmc

pub struct ConvBmc;

impl DbBmc for ConvBmc {
	const TABLE: &'static str = "conv";
}

gen_mm_crud_fns!(
	Bmc: ConvBmc,
	ForGet: Conv,
	ForCreate: ConvForCreate,
	ForUpdate: ConvForUpdate,
	ForList: Conv,
	Filter: ConvFilter,
);

// endregion: --- ConvBmc

// region:    --- ConvMsg Types & Bmc Fns

/// Outside face message type to be used outside of the model layer.
/// This will get translated to a `cfile_db::Msg`
#[derive(Serialize)]
pub struct ConvMsg {
	// Note: will be empty string if not complete yet.
	pub content: String,
}

// impl from String
impl From<String> for ConvMsg {
	fn from(s: String) -> Self {
		Self { content: s }
	}
}

// impl for str
impl From<&str> for ConvMsg {
	fn from(s: &str) -> Self {
		Self { content: s.to_string() }
	}
}

impl ConvMsg {
	pub fn from_msg(msg: Msg) -> Self {
		Self {
			content: msg.content.unwrap_or_default(),
		}
	}
}

impl ConvBmc {
	pub async fn getc_cfile_db_for_conv_id(mm: &ModelManager, conv_id: Id) -> Result<SlDb> {
		let conv = ConvBmc::get(mm, conv_id).await?;
		let cfile_db = CFileBmc::getc_cfile_db_for_conv(mm, &conv).await?;
		Ok(cfile_db)
	}

	pub async fn getc_cfile_db(mm: &ModelManager, conv: &Conv) -> Result<SlDb> {
		let cfile_db = CFileBmc::getc_cfile_db_for_conv(mm, conv).await?;
		Ok(cfile_db)
	}

	pub async fn get_agent_uid_for_msg_id(mm: &ModelManager, cfile_db: &SlDb, msg_id: Id) -> Result<String> {
		let conv_uid = MsgBmc::get_conv_uid(cfile_db, msg_id).await?;

		// get agent_uid from conv_uid
		let sql = r#"
SELECT agent.uid
FROM conv
JOIN space ON conv.space_id = space.id
JOIN agent ON space.agent_id = agent.id
WHERE conv.uid = ?1;  -- Replace '?' with the specific conv.uid you are looking for
	"#;

		let agent_uid = mm.main_db().exec_returning_as(sql, (conv_uid,))?;
		Ok(agent_uid)
	}

	pub async fn add_conv_msg(mm: &ModelManager, conv_id: Id, conv_msg: ConvMsg) -> Result<Id> {
		let conv = ConvBmc::get(mm, conv_id).await?;

		// -- get the cfile_db
		let cfile_db = CFileBmc::getc_cfile_db_for_conv(mm, &conv).await?;

		// -- Get the conv_ref
		let conv_ref_id = ConvRefBmc::get_or_create_conv_ref_id(mm, &cfile_db, &conv).await?;

		// -- Create the cfile_db Msg
		let msg_c = MsgForCreate::from_conv_msg(conv_ref_id, conv_msg);
		let msg_id = MsgBmc::create_user_question(&cfile_db, msg_c).await?;

		// Create the original stack_step
		let mut _step_id = StackStepBmc::create_first_from_msg_id(&cfile_db, msg_id).await?;

		// TODO: Need to check if we want to call this here.
		Self::touch_work_tnew(mm, conv_id).await?;

		Ok(msg_id)
	}

	pub async fn clear_all(mm: &ModelManager, conv_id: Id) -> Result<()> {
		let conv = ConvBmc::get(mm, conv_id).await?;
		let cfile_db = CFileBmc::getc_cfile_db_for_conv(mm, &conv).await?;
		if let Some(conv_ref_id) = conv.conv_ref_id {
			// NOTE: here we do not rely on the stack_step FK on msg for now.
			//       We might change this strategy, but right now, those FKs migh go away later
			//       As they might add too much constrain on creation/import of other db
			//       and in SQLite cannot be added to existing table (forcing to create a new table and migrate data)

			// -- Delete the stack_step
			let sql = r#"
DELETE FROM stack_step
WHERE orig_msg_id IN (
    SELECT id FROM msg WHERE conv_ref_id = ?
);
"#;
			cfile_db.exec(sql, [conv_ref_id.as_i64()])?;

			// -- Delete the msg
			let sql = "DELETE FROM msg where conv_ref_id = ?";
			cfile_db.exec(sql, [conv_ref_id.as_i64()])?;
		} else {
			error!("Conv[{}] does not have a conv_ref_id", conv.id);
		}

		Ok(())
	}

	pub async fn list_msgs(mm: &ModelManager, conv_id: Id) -> Result<Vec<Msg>> {
		let conv = ConvBmc::get(mm, conv_id).await?;
		let cfile_db = CFileBmc::getc_cfile_db_for_conv(mm, &conv).await?;

		// TODO: Need to handle the case it there is none. perhaps an error.
		if let Some(conv_ref_id) = conv.conv_ref_id {
			let filter = MsgFilter {
				conv_ref_id: Some(conv_ref_id.as_i64().into()),
				..Default::default()
			};

			let msgs = MsgBmc::list(&cfile_db, Some(vec![filter]), None).await?;
			// let conv_msgs = msgs.into_iter().map(ConvMsg::from_msg).collect();

			Ok(msgs)
		} else {
			Ok(Vec::new())
		}
	}

	pub async fn list_steps(mm: &ModelManager, conv_id: Id, orig_msg_id: Id) -> Result<Vec<StackStepLite>> {
		let conv = ConvBmc::get(mm, conv_id).await?;
		let cfile_db = CFileBmc::getc_cfile_db_for_conv(mm, &conv).await?;

		// TODO: Need to handle the case it there is none. perhaps an error.
		if let Some(conv_ref_id) = conv.conv_ref_id {
			let filter = StackStepFilter {
				orig_msg_id: Some(orig_msg_id.as_i64().into()),
				..Default::default()
			};

			let msgs = StackStepBmc::list(&cfile_db, Some(vec![filter]), None).await?;
			// let conv_msgs = msgs.into_iter().map(ConvMsg::from_msg).collect();

			Ok(msgs)
		} else {
			Ok(Vec::new())
		}
	}

	pub async fn get_step(mm: &ModelManager, conv_id: Id, step_id: Id) -> Result<StackStep> {
		let cfile_db = ConvBmc::getc_cfile_db_for_conv_id(mm, conv_id).await?;
		let step = StackStepBmc::get(&cfile_db, step_id).await?;

		Ok(step)
	}

	// pub async seek

	// region:    --- TWork

	pub async fn touch_work_tnew(mm: &ModelManager, conv_id: Id) -> Result<()> {
		let field = SeaField::siden(Conv::WORK_TNEW, now());

		base::update_with_fields::<Self>(mm.main_db(), conv_id, field.into()).await?;

		// Note: Here we do not handle error for now.
		//       Eventually, might be better to have this function not return Result, and
		//       have it handle the error inside (e.g., tracing)
		mm.hub().publish(ConvEvent::ConvWorkNew { conv_id }).await;

		Ok(())
	}

	pub async fn touch_work_tdone(mm: &ModelManager, conv_id: Id) -> Result<()> {
		let field = SeaField::siden(Conv::WORK_TDONE, now());

		base::update_with_fields::<Self>(mm.main_db(), conv_id, field.into()).await?;

		// Note: Same as above, do not need to handle the error here.
		mm.hub().publish(ConvEvent::ConvWorkDone { conv_id }).await;

		Ok(())
	}

	pub async fn touch_work_both(mm: &ModelManager, conv_id: Id) -> Result<()> {
		let now = now();

		let fields = vec![SeaField::siden(Conv::WORK_TNEW, now), SeaField::siden(Conv::WORK_TDONE, now)];
		base::update_with_fields::<Self>(mm.main_db(), conv_id, fields.into()).await?;
		Ok(())
	}

	// endregion: --- TWork
}

// endregion: --- ConvMsg Types & Bmc Fns
