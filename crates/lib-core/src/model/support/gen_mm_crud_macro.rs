/// Generate the BMC CRUD implementation with ModelManager db context.
#[macro_export]
macro_rules! gen_mm_crud_fns {
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
        impl $struct_name {
            $(
                pub async fn create(mm: &ModelManager, entity_c: $for_create) -> Result<Id> {
                    base::create::<Self, _>(mm.main_db(), entity_c).await
                }
            )?
            $(
                pub async fn get(mm: &ModelManager, id: Id) -> Result<$for_get> {
                    base::get::<Self, _>(mm.main_db(), id).await
                }
            )?
            $(
                pub async fn get_by_uid(mm: &ModelManager, uid: &str) -> Result<$for_get_uid> {
                    base::get_by_uid::<Self, _>(mm.main_db(), uid).await
                }
            )?
            $(
                pub async fn first(
                    mm: &ModelManager,
                    filter: Option<Vec<$filter>>,
                    list_options: Option<ListOptions>,
                ) -> Result<Option<$for_list>> {
                    base::first::<Self, _, _>(mm.main_db(), filter, list_options).await
                }

                pub async fn list(
                    mm: &ModelManager,
                    filter: Option<Vec<$filter>>,
                    list_options: Option<ListOptions>,
                ) -> Result<Vec<$for_list>> {
                    base::list::<Self, _, _>(mm.main_db(), filter, list_options).await
                }
            )?

            $(
                pub async fn update(mm: &ModelManager, id: Id, entity_u: $for_update) -> Result<()> {
                    base::update::<Self, _>(mm.main_db(), id, entity_u).await
                }
            )?

            pub async fn delete(mm: &ModelManager, id: Id) -> Result<()> {
                base::delete::<Self>(mm.main_db(), id).await
            }
        }
    };
}
