// region:    --- Modules

mod crud_fns;
mod db_bmc;
mod id;
mod prep_fields;
mod sea_iden;

// -- Flatten module hierarchy for user code
pub use crud_fns::*;
pub use db_bmc::*;
pub use id::*;
pub use sea_iden::*;

// endregion: --- Modules

// -- Common Constants

const LIST_LIMIT_DEFAULT: i64 = 1000;
const LIST_LIMIT_MAX: i64 = 5000;
