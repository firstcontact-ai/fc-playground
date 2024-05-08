use crate::model::DbType;

pub struct DbConfig {
	pub schema: &'static str,
	pub file_name: &'static str,
	pub dir: &'static str,
	pub table_check: &'static str,
}

pub fn get_db_config(db_type: &DbType) -> &'static DbConfig {
	static MAIN_DB_CONFIG: DbConfig = DbConfig {
		schema: include_str!("schemas/main-db-schema.sql"),
		file_name: "fc-main.db3",
		dir: "",
		table_check: "space",
	};

	static DFILE_DB_CONFIG: DbConfig = DbConfig {
		schema: include_str!("schemas/dfile-db-schema.sql"),
		file_name: "dfile-{uid}.db3",
		dir: "dfiles",
		table_check: "part",
	};

	static CFILE_DB_CONFIG: DbConfig = DbConfig {
		schema: include_str!("schemas/cfile-db-schema.sql"),
		file_name: "cfile-{uid}.db3",
		dir: "cfiles",
		table_check: "msg",
	};

	match db_type {
		DbType::Main => &MAIN_DB_CONFIG,
		DbType::DFile => &DFILE_DB_CONFIG,
		DbType::CFile => &CFILE_DB_CONFIG,
	}
}
