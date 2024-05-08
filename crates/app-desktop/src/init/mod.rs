// region:    --- Modules

// -- Privates
mod agents_initializer;
mod spaces_initializer;

// -- Imports
use crate::init::agents_initializer::init_agents;
use crate::init::spaces_initializer::init_spaces;
use crate::Result;
use lib_core::model::ModelManager;

// endregion: --- Modules

pub async fn init_main_db_all(mm: &ModelManager) -> Result<()> {
	init_agents(mm).await?;
	init_spaces(mm).await?;

	Ok(())
}
