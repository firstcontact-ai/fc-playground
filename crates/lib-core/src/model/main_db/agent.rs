use crate::model::support::prelude::*;
use derive_more::Display;
use modql::field::SeaFieldValue;
use modql::FromSqliteValue;

// region:    --- Types

#[cfg_attr(feature = "for-ts", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, SeaFieldValue, FromSqliteValue, Serialize, Deserialize, Display)]
pub enum OutFormat {
	Text,
	Json,
}

#[cfg_attr(feature = "for-ts", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, SeaFieldValue, FromSqliteValue, Serialize, Deserialize, Display)]
pub enum AgentKind {
	Ai,
	Logic,
}

#[derive(Debug, Clone, Fields, FromSqliteRow, Serialize, Deserialize)]
#[cfg_attr(feature = "for-ts", derive(schemars::JsonSchema))]
pub struct Agent {
	pub id: Id,
	pub uid: String,

	pub name: String,
	pub kind: AgentKind,
	pub desc: Option<String>,

	pub space_default: bool,

	pub provider: Option<String>,
	pub model: Option<String>,
	pub inst: Option<String>,
	pub prompt_tmpl: Option<String>,
	pub chain: Option<String>,

	pub out_format: Option<OutFormat>,
}

impl Agent {}

#[derive(Debug, Clone, Fields, FromSqliteRow, Serialize, Deserialize)]
#[cfg_attr(feature = "for-ts", derive(schemars::JsonSchema))]
pub struct AgentLite {
	pub id: Id,

	pub name: String,
}

#[derive(Fields, Deserialize)]
#[cfg_attr(feature = "for-ts", derive(schemars::JsonSchema))]
pub struct AgentForCreate {
	pub name: String,
	pub kind: Option<AgentKind>,
	pub desc: Option<String>,

	pub space_default: Option<bool>,

	pub provider: Option<String>,
	pub model: Option<String>,
	pub inst: Option<String>,
	pub prompt_tmpl: Option<String>,
	pub out_format: Option<OutFormat>,
}

impl AgentForCreate {
	pub fn new(name: impl Into<String>) -> AgentForCreate {
		Self {
			name: name.into(),
			kind: None,
			desc: None,
			space_default: None,
			provider: None,
			model: None,
			inst: None,
			prompt_tmpl: None,
			out_format: None,
		}
	}
}

#[derive(Fields, Default, Deserialize)]
#[cfg_attr(feature = "for-ts", derive(schemars::JsonSchema))]
pub struct AgentForUpdate {
	pub name: Option<String>,
	pub desc: Option<String>,

	pub space_default: Option<bool>,

	pub provider: Option<String>,
	pub model: Option<String>,
	pub inst: Option<String>,
	pub prompt_tmpl: Option<String>,
	pub chain: Option<String>,
	pub out_format: Option<OutFormat>,
}

#[derive(FilterNodes, Default, Deserialize)]
pub struct AgentFilter {
	pub id: Option<OpValsInt64>,

	pub name: Option<OpValsString>,
	pub kind: Option<OpValsString>,

	pub space_default: Option<OpValsBool>,
}

// implement default for AGentFilter

// endregion: --- Types

// region:    --- AgentBmc

pub struct AgentBmc;

impl DbBmc for AgentBmc {
	const TABLE: &'static str = "agent";
}

gen_mm_crud_fns!(
	Bmc: AgentBmc,
	ForGet: Agent,
	ForGetByUid: Agent,
	ForCreate: AgentForCreate,
	ForUpdate: AgentForUpdate,
);

/// NOTE: Right now, just custom filter and first fo the filter Ai, but this my change later.
impl AgentBmc {
	pub async fn first_by_name(mm: &ModelManager, name: &str) -> Result<Option<Agent>> {
		AgentBmc::first(
			mm,
			Some(vec![AgentFilter {
				name: Some(name.to_string().into()),
				..Default::default()
			}]),
			None,
		)
		.await
	}
	pub async fn list(
		mm: &ModelManager,
		filters: Option<Vec<AgentFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<AgentLite>> {
		// -- Make sure to add `AgentKind::Ai` to all filters
		let filters = lock_filter_to_agent_kind(filters, AgentKind::Ai);

		// -- Exec the list
		base::list::<Self, _, _>(mm.main_db(), filters, list_options).await
	}

	pub async fn first(
		mm: &ModelManager,
		filters: Option<Vec<AgentFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Option<Agent>> {
		// -- Make sure to add `AgentKind::Ai` to all filters
		let filters = lock_filter_to_agent_kind(filters, AgentKind::Ai);

		// -- Exec the list
		base::first::<Self, _, _>(mm.main_db(), filters, list_options).await
	}

	pub async fn seek_space_default_agent(mm: &ModelManager) -> Result<Option<Agent>> {
		let filter = AgentFilter {
			space_default: Some(true.into()),
			..Default::default()
		};
		let agent = Self::first(mm, Some(vec![filter]), None).await?;

		Ok(agent)
	}
}

fn lock_filter_to_agent_kind(filters: Option<Vec<AgentFilter>>, kind: AgentKind) -> Option<Vec<AgentFilter>> {
	filters
		.map(|mut filters| {
			filters.iter_mut().for_each(|f| f.kind = Some(kind.to_string().into()));
			filters
		})
		.or_else(|| {
			Some(vec![AgentFilter {
				kind: Some(kind.to_string().into()),
				..Default::default()
			}])
		})
}

// endregion: --- AgentBmc

// region:    --- AgentError

pub type AgentResult<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum AgentError {
	SystemAgentNotFound { name: String },
}

// region:    --- Error Boilerplate

impl core::fmt::Display for AgentError {
	fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for AgentError {}

// endregion: --- Error Boilerplate

// endregion: --- AgentError

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Error = Box<dyn std::error::Error>;
	type Result<T> = core::result::Result<T, Error>; // For tests.

	use super::*;
	use modql::filter::OpValString;

	#[tokio::test]
	async fn test_create_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = ModelManager::new().await?;
		let fx_name = "test-agent 01".to_string();

		// -- Exec
		let id = AgentBmc::create(&mm, AgentForCreate::new(fx_name)).await?;

		// -- Check
		assert_eq!(*id, 1);

		Ok(())
	}

	#[tokio::test]
	async fn test_list_ok_simple() -> Result<()> {
		// -- Setup & Fixtures
		let mm = ModelManager::new().await?;
		let fx_names: &[&str] = &["test_list 01", "test_list 02", "test_list 03"];
		for &name in fx_names {
			AgentBmc::create(&mm, AgentForCreate::new(name)).await?;
		}

		// -- Exec
		let items = AgentBmc::list(&mm, None, None).await?;

		// -- Check
		let names: Vec<&str> = items.iter().map(|a| a.name.as_ref()).collect();
		assert_eq!(&names, fx_names);

		Ok(())
	}

	#[tokio::test]
	async fn test_list_ok_filter() -> Result<()> {
		// -- Setup & Fixtures
		let mm = ModelManager::new().await?;
		let fx_names_a: &[&str] = &["test_list A-01", "test_list A-02.1", "test_list A-02.2"];
		for &name in fx_names_a {
			AgentBmc::create(&mm, AgentForCreate::new(name)).await?;
		}
		let fx_names_b: &[&str] = &["test_list B-01", "test_list B-02"];
		for &name in fx_names_b {
			AgentBmc::create(&mm, AgentForCreate::new(name)).await?;
		}

		// -- Exec
		let filter = AgentFilter {
			name: Some(OpValString::Contains("A-02".to_string()).into()),
			..Default::default()
		};
		let items = AgentBmc::list(&mm, Some(vec![filter]), None).await?;

		// -- Check
		let names: Vec<&str> = items.iter().map(|a| a.name.as_ref()).collect();
		assert_eq!(&names, &fx_names_a[1..=2]);

		Ok(())
	}
}

// endregion: --- Tests
