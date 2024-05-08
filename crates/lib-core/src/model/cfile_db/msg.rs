use crate::model::conv::ConvMsg;
use crate::model::stack_step::StackStep;
use crate::model::support::prelude::*;

// region:    --- Types

#[cfg_attr(feature = "for-ts", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, SeaFieldValue, FromSqliteValue, Serialize, Deserialize)]
pub enum AuthorKind {
	Agent,
	User,
}

#[derive(Debug, Clone, Fields, FromSqliteRow, Serialize, Deserialize)]
pub struct Msg {
	pub id: Id,
	pub uid: String,

	pub conv_ref_id: Id,
	pub orig_msg_id: Option<Id>,

	pub author_kind: Option<AuthorKind>,

	pub content: Option<String>,

	pub ctime: UnixTimeUs,
	pub mtime: UnixTimeUs,
}

#[derive(Debug, Clone, Fields, FromSqliteRow, Serialize, Deserialize)]
pub struct MsgForCreate {
	pub conv_ref_id: Id,
	pub orig_msg_id: Option<Id>,

	pub author_kind: AuthorKind,

	pub content: Option<String>,
}

impl MsgForCreate {
	pub fn from_conv_msg(conv_ref_id: Id, conv_msg: ConvMsg) -> Self {
		let content = if conv_msg.content.is_empty() {
			None
		} else {
			Some(conv_msg.content)
		};

		Self {
			conv_ref_id,
			orig_msg_id: None,
			author_kind: AuthorKind::User,
			content,
		}
	}
}

#[derive(FilterNodes, Default, Deserialize)]
pub struct MsgFilter {
	pub id: Option<OpValsInt64>,

	pub conv_ref_id: Option<OpValsInt64>,
}

// endregion: --- Types

// region:    --- Bmc

pub struct MsgBmc;

impl DbBmc for MsgBmc {
	const TABLE: &'static str = "msg";
}

generate_sldb_crud_fns!(
	Bmc: MsgBmc,
	ForGet: Msg,
	ForList: Msg,
	Filter: MsgFilter,
);

impl MsgBmc {
	pub async fn get_conv_uid(db: &SlDb, msg_id: Id) -> Result<String> {
		let sql = r#"
SELECT conv_ref.conv_uid
FROM conv_ref
JOIN msg ON conv_ref.id = msg.conv_ref_id
WHERE msg.id = ?1
		"#;
		let conv_uid = db.exec_returning_as::<String>(sql, (msg_id,))?;
		Ok(conv_uid)
	}

	pub async fn create_user_question(db: &SlDb, msg_c: MsgForCreate) -> Result<Id> {
		// check that author_kind is User
		if !matches!(msg_c.author_kind, AuthorKind::User) {
			return Err(MsgError::UserQuestionFailNoAuthorUser.into());
		}

		base::create::<Self, _>(db, msg_c).await
	}

	pub async fn create_agent_answer(db: &SlDb, closer_step: StackStep, content: impl Into<String>) -> Result<Id> {
		let orig_msg = Self::get(db, closer_step.orig_msg_id).await?;

		let msg_c = MsgForCreate {
			conv_ref_id: orig_msg.conv_ref_id,
			orig_msg_id: Some(orig_msg.id),
			author_kind: AuthorKind::Agent,
			content: Some(content.into()),
		};

		base::create::<Self, _>(db, msg_c).await
	}
}

// endregion: --- Bmc

// region:    --- MsgError

#[derive(Debug, Serialize)]
pub enum MsgError {
	UserQuestionFailNoAuthorUser,
}

// region:    --- Error Boilerplate

impl core::fmt::Display for MsgError {
	fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for MsgError {}

// endregion: --- Error Boilerplate

// endregion: --- MsgError
