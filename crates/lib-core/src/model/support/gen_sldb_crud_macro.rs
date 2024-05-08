/// Generate the BMC CRUD implementaiton with SlDb as input
#[macro_export]
macro_rules! generate_sldb_crud_fns {
    (
        Bmc: $struct_name:ident,
        $(ForGet: $for_get:ty,)?
        $(ForGetByUid: $for_get_uid:ty,)?
        $(ForCreate: $for_create:ty,)?
        $(ForUpdate: $for_update:ty,)?
        $(
        ForList: $for_list:ty,
        Filter: $filter:ty,
        )?
    ) => {
        #[allow(unused)] // Note: Some used method do not get marked as such. So, disabling warning.
        impl $struct_name {
            $(
                pub async fn create(db: &SlDb, entity_c: $for_create) -> Result<Id> {
                    base::create::<Self, _>(db, entity_c).await
                }
            )?
            $(
                pub async fn get(db: &SlDb, id: Id) -> Result<$for_get> {
                    base::get::<Self, _>(db, id).await
                }
            )?
            $(
                pub async fn get_by_uid(db: &SlDb, uid: &str) -> Result<$for_get_uid> {
                    base::get_by_uid::<Self, _>(db, uid).await
                }
            )?
            $(
                pub async fn first(
                    db: &SlDb,
                    filter: Option<Vec<$filter>>,
                    list_options: Option<ListOptions>,
                ) -> Result<Option<$for_list>> {
                    base::first::<Self, _, _>(db, filter, list_options).await
                }

                pub async fn list(
                    db: &SlDb,
                    filter: Option<Vec<$filter>>,
                    list_options: Option<ListOptions>,
                ) -> Result<Vec<$for_list>> {
                    base::list::<Self, _, _>(db, filter, list_options).await
                }
            )?

            $(
                pub async fn update(db: &SlDb, id: Id, entity_u: $for_update) -> Result<()> {
                    base::update::<Self, _>(db, id, entity_u).await
                }
            )?

            pub async fn delete(db: &SlDb, id: Id) -> Result<()> {
                base::delete::<Self>(db, id).await
            }
        }
    };
}
