use sea_query::Iden;

#[derive(Iden)]
pub enum CommonIden {
	Id,
	Uid,
	LastOpen,

	// -- entity ids
	// Note: different case to have the right snake case
	DsourceId,
}

#[derive(Iden)]
pub enum TimestampIden {
	Ctime,
	Mtime,
}
