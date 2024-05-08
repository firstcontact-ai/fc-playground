use crate::model::cfile_db::msg::Msg;
use crate::model::msg::MsgBmc;
use crate::model::support::prelude::*;
use lib_utils::time::now;
use modql::field::{HasSeaFields, SeaField};

// region:    --- Types

#[derive(Debug, Clone, Fields, FromSqliteRow, Serialize, Deserialize)]
#[modql(names_as_consts)]
pub struct StackStep {
	pub id: Id,
	pub uid: String,

	pub orig_msg_id: Id,

	pub first_step_id: Option<Id>,
	pub prev_step_id: Option<Id>,
	pub closer: bool,

	// -- Call Ctx
	pub call_stack: Option<String>,
	pub call_out: Option<String>,
	pub call_err: Option<String>,

	pub resolve_tstart: Option<UnixTimeUs>,
	pub resolve_tend: Option<UnixTimeUs>,
	pub resolve_model: Option<String>,

	pub run_agent_uid: Option<String>,
	pub run_agent_name: Option<String>,

	pub run_tstart: Option<UnixTimeUs>,
	pub run_tend: Option<UnixTimeUs>,
	pub run_terr: Option<UnixTimeUs>,

	pub ctime: UnixTimeUs,
	pub mtime: UnixTimeUs,
}

#[derive(Debug, Clone, Fields, FromSqliteRow, Serialize, Deserialize)]
#[modql(names_as_consts)]
pub struct StackStepLite {
	pub id: Id,

	pub uid: String,
	pub orig_msg_id: Id,

	pub first_step_id: Option<Id>,
	pub prev_step_id: Option<Id>,
	pub closer: bool,
}

#[derive(Debug, Clone, Fields, FromSqliteRow, Serialize, Deserialize)]
pub struct StackStepForCreate {
	pub orig_msg_id: Id,
	pub first_step_id: Option<Id>,
	pub prev_step_id: Option<Id>,
}

impl StackStepForCreate {
	/// Note: here we do not implement From trait as we will probably need more context later
	pub fn from_msg(msg: Msg) -> Self {
		Self {
			orig_msg_id: msg.id,
			first_step_id: None, // will be updated on create
			prev_step_id: None,
		}
	}

	pub fn from_prev_step(step: StackStep) -> Self {
		Self {
			orig_msg_id: step.orig_msg_id,
			first_step_id: step.first_step_id,
			prev_step_id: Some(step.id),
		}
	}
}

#[derive(Debug, Clone, Default, Fields, FromSqliteRow, Serialize, Deserialize)]
pub struct StackStepForUpdate {
	pub resolve_model: Option<String>,
	pub run_agent_uid: Option<String>,
	pub run_agent_name: Option<String>,
	pub call_stack: Option<String>,
	pub call_out: Option<String>,
	pub call_err: Option<String>,
}

#[derive(FilterNodes, Default, Deserialize)]
pub struct StackStepFilter {
	pub id: Option<OpValsInt64>,

	pub orig_msg_id: Option<OpValsInt64>,

	// -- Call Ctx
	pub call_stack: Option<OpValsString>,
	pub call_input: Option<OpValsString>,
	pub call_out: Option<OpValsString>,
	pub call_err: Option<OpValsString>,

	pub resolve_tstart: Option<OpValsInt64>,
	pub run_tstart: Option<OpValsInt64>,
}

// endregion: --- Types

// region:    --- Bmc

pub struct StackStepBmc;

impl DbBmc for StackStepBmc {
	const TABLE: &'static str = "stack_step";
}

generate_sldb_crud_fns!(
	Bmc: StackStepBmc,
	ForGet: StackStep,
	// NOTE: Here we do not do the ForUpdate, because we want to control the various updates
	ForList: StackStepLite,
	Filter: StackStepFilter,
);

impl StackStepBmc {
	pub async fn create_first_from_msg_id(db: &SlDb, msg_id: Id) -> Result<Id> {
		let msg = MsgBmc::get(db, msg_id).await?;
		let stack_step_c = StackStepForCreate::from_msg(msg);

		let id = base::create::<Self, _>(db, stack_step_c).await?;

		// IMPORTANT: need to set the step.first_step_id to this one
		let field = SeaField::siden(StackStep::FIRST_STEP_ID, id.as_i64());
		base::update_with_fields::<Self>(db, id, field.into()).await?;

		Ok(id)
	}

	pub async fn create_next_from_step(db: &SlDb, step_id: Id) -> Result<Id> {
		let step = StackStepBmc::get(db, step_id).await?;
		let next_step_c = StackStepForCreate::from_prev_step(step);
		let next_step_id = base::create::<Self, _>(db, next_step_c).await?;
		Ok(next_step_id)
	}

	// region:    --- Update Methods

	pub async fn update_resolve_success(
		db: &SlDb,
		step_id: Id,
		step_u: StackStepForUpdate,
		is_closer: bool,
	) -> Result<()> {
		let fields = step_u
			.not_none_sea_fields()
			.append_siden(StackStep::RESOLVE_TEND, now())
			.append_siden(StackStep::CLOSER, is_closer);
		base::update_with_fields::<Self>(db, step_id, fields).await?;

		Ok(())
	}

	pub async fn update_run_end_success(db: &SlDb, step_id: Id, step_u: StackStepForUpdate) -> Result<()> {
		let fields = step_u.not_none_sea_fields().append_siden(StackStep::RUN_TEND, now());
		base::update_with_fields::<Self>(db, step_id, fields).await?;

		Ok(())
	}

	pub async fn update_run_end_fail(db: &SlDb, step_id: Id, step_u: StackStepForUpdate) -> Result<()> {
		let fields = step_u
			.not_none_sea_fields()
			.append_siden(StackStep::RUN_TEND, now())
			.append_siden(StackStep::RUN_TERR, now());
		base::update_with_fields::<Self>(db, step_id, fields).await?;

		Ok(())
	}

	// endregion: --- Update Methods

	// region:    --- Update "T-States"

	pub async fn set_step_as_closer(db: &SlDb, step_id: Id) -> Result<()> {
		let field = SeaField::siden(StackStep::CLOSER, true);
		base::update_with_fields::<Self>(db, step_id, field.into()).await?;
		Ok(())
	}

	pub async fn set_resolve_tstart(db: &SlDb, step_id: Id) -> Result<()> {
		let field = SeaField::siden(StackStep::RESOLVE_TSTART, now());
		base::update_with_fields::<Self>(db, step_id, field.into()).await?;
		Ok(())
	}

	pub async fn set_run_tstart(db: &SlDb, step_id: Id) -> Result<()> {
		let field = SeaField::siden(StackStep::RUN_TSTART, now());
		base::update_with_fields::<Self>(db, step_id, field.into()).await?;
		Ok(())
	}

	pub async fn set_run_tend(db: &SlDb, step_id: Id) -> Result<()> {
		let field = SeaField::siden(StackStep::RUN_TEND, now());
		base::update_with_fields::<Self>(db, step_id, field.into()).await?;
		Ok(())
	}

	// endregion: --- Update "T-States"

	/// This will get the next StackStep to be taken to work on (e.g., `started == null`)
	pub async fn seek_next_to_resolve_for_conv(db: &SlDb, conv_uid: &str) -> Result<Option<StackStep>> {
		let sql = r#"
SELECT s.id
  FROM stack_step s
  JOIN msg m ON s.orig_msg_id = m.id
  JOIN conv_ref c ON m.conv_ref_id = c.id
 WHERE c.conv_uid = ?
   AND s.resolve_tstart IS NULL
 ORDER BY s.id ASC
 LIMIT 1;
		"#;
		let res: Option<Id> = db.exec_returning_as_optional(sql, (conv_uid,))?;

		if let Some(stack_id) = res {
			let step = Self::get(db, stack_id).await?;
			Ok(Some(step))
		} else {
			Ok(None)
		}
	}

	///
	pub async fn seek_next_to_run_for_conv(db: &SlDb, conv_uid: &str) -> Result<Option<StackStep>> {
		let sql = r#"
SELECT s.*
  FROM stack_step s
  JOIN msg ON s.orig_msg_id = msg.id
  JOIN conv_ref ON msg.conv_ref_id = conv_ref.id
 WHERE conv_ref.conv_uid = ?
   AND s.resolve_tend IS NOT NULL
   AND s.run_tstart IS NULL;
 ORDER BY s.id ASC
 LIMIT 1;
		"#;
		let res: Option<Id> = db.exec_returning_as_optional(sql, (conv_uid,))?;

		if let Some(stack_id) = res {
			let step = Self::get(db, stack_id).await?;
			Ok(Some(step))
		} else {
			Ok(None)
		}
	}

	pub async fn get_prev_step_call_out(db: &SlDb, step_id: Id) -> Result<Option<String>> {
		let sql = r#"
SELECT ss2.call_out
FROM stack_step ss1
JOIN stack_step ss2 ON ss1.prev_step_id = ss2.id
WHERE ss1.id = ?;		
		"#;
		let out: Option<String> = db.exec_returning_as(sql, (step_id,))?;

		Ok(out)
	}
}

// endregion: --- Bmc
