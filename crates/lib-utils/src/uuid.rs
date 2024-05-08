use uuid::Uuid;

pub fn new_uuid() -> Uuid {
	Uuid::now_v7()
}

/// Create a new uuid v7, and base32hex with no pad encode it.
pub fn new_uuid_b32x() -> String {
	data_encoding::BASE32HEX_NOPAD.encode(new_uuid().as_bytes())
}

pub fn uuid_from_b32x(b32x_no_bpd: &str) -> Result<Uuid> {
	let b32x_bytes = data_encoding::BASE32HEX_NOPAD
		.decode(b32x_no_bpd.as_bytes())
		.map_err(|_| Error::FailToB32Decode)?;

	let b32x_bytes: [u8; 16] = b32x_bytes
		.try_into()
		.map_err(|_| "Not a [u8; 16] vec")
		.map_err(|_| Error::B32BytesNot16u8)?;

	let uuid = Uuid::from_bytes(b32x_bytes);

	Ok(uuid)
}

// region:    --- Error

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	// TBC
	FailToB32Decode,
	B32BytesNot16u8,
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
