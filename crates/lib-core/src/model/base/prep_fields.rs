use crate::model::base::{CommonIden, DbBmc, TimestampIden};
use lib_utils::time::now_unix_time_us;
use lib_utils::uuid::new_uuid_b32x;
use modql::field::{SeaField, SeaFields};
use sea_query::IntoIden;

// NOTES
//   - Make sure to use the `now_utc_fmt` for Rfc3339 for time, otherwise, rusqlite format it wrong.

/// This method must be called when a model controller intends to create its entity.
pub fn prep_fields_for_create<MC>(fields: &mut SeaFields)
where
	MC: DbBmc,
{
	if MC::has_timestamps() {
		add_timestamps_for_create(fields);
	}
	if MC::has_uid() {
		fields.push(SeaField::new(CommonIden::Uid, new_uuid_b32x()))
	}
	if MC::set_last_open_on_create() {
		let now = now_unix_time_us();
		fields.push(SeaField::new(CommonIden::LastOpen, now));
	}
}

/// This method must be calledwhen a Model Controller plans to update its entity.
pub fn prep_fields_for_update<MC>(fields: &mut SeaFields)
where
	MC: DbBmc,
{
	if MC::has_timestamps() {
		add_timestamps_for_update(fields);
	}
}

/// Update the timestamps info for create
/// (e.g., cid, ctime, and mid, mtime will be updated with the same values)
fn add_timestamps_for_create(fields: &mut SeaFields) {
	let now = now_unix_time_us();
	fields.push(SeaField::new(TimestampIden::Ctime.into_iden(), now));
	fields.push(SeaField::new(TimestampIden::Mtime.into_iden(), now));
}

/// Update the timestamps info only for update.
/// (.e.g., only mid, mtime will be udpated)
fn add_timestamps_for_update(fields: &mut SeaFields) {
	let now = now_unix_time_us();
	fields.push(SeaField::new(TimestampIden::Mtime.into_iden(), now));
}
