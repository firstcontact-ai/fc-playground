use derive_more::From;
use handlebars::Handlebars;
use serde::Serialize;
use std::sync::OnceLock;

pub fn render<T>(tmpl: &str, data: T) -> Result<String>
where
	T: Serialize,
{
	let hbs_reg = get_handlebars();
	let res = hbs_reg.render_template(tmpl, &data)?;
	Ok(res)
}

/// get_hbs_reg() that return the default handlebars using OnceLock
pub fn get_handlebars() -> &'static Handlebars<'static> {
	static INSTANCE: OnceLock<Handlebars> = OnceLock::new();
	INSTANCE.get_or_init(Handlebars::new)
}

// region:    --- Error

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
	// -- Externals
	#[from]
	Handlebars(handlebars::RenderError), // as example
}

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
	fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate

// endregion: --- Error
