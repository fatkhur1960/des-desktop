use super::{EntriesResult, Error as ServiceError, IdPayload, PayloadEntries};
use serde::{Deserialize, Serialize};

use crate::{dao::item_dao::ItemDao, error::ErrorCode, models, ApiResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddItem {
    pub item_name: String,
    pub item_desc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateItem {
    pub id: i32,
    pub item_name: Option<String>,
    pub item_desc: Option<String>,
}

#[derive(Default)]
pub struct ItemService;

#[service]
impl ItemService {
    #[route]
    pub fn add_item(payload: AddItem) -> ApiResult<String> {
        let conn = state.db();
        let dao = ItemDao::new(&conn);
        dao.create(
            &payload.item_name,
            &payload.item_desc.unwrap_or("".to_string()),
        )?;

        Ok(ApiResult::success(format!(
            "{} disimpan",
            &payload.item_name
        )))
    }

    #[route]
    pub fn update_item(payload: UpdateItem) -> ApiResult<String> {
        let conn = state.db();
        let dao = ItemDao::new(&conn);
        let item = dao.get_by_id(payload.id)?;

        dao.update(
            payload.id,
            &payload.item_name.unwrap_or(item.item_name.to_owned()),
            &payload.item_desc.unwrap_or(item.item_desc),
        )?;

        Ok(ApiResult::success(format!(
            "{} di update",
            &item.item_name.clone()
        )))
    }

    #[route]
    pub fn get_items(payload: PayloadEntries) -> ApiResult<EntriesResult<models::Item>> {
        let conn = state.db();
        let dao = ItemDao::new(&conn);

        let (entries, count) = dao.get_items(payload.query, payload.offset, payload.limit)?;

        Ok(ApiResult::success(EntriesResult { entries, count }))
    }

    #[route]
    pub fn get_item(payload: IdPayload) -> ApiResult<models::Item> {
        let conn = state.db();
        let dao = ItemDao::new(&conn);

        let item = dao.get_by_id(payload.id)?;

        Ok(ApiResult::success(item))
    }

    #[route]
    pub fn delete_item(payload: IdPayload) -> ApiResult<String> {
        let conn = state.db();
        let dao = ItemDao::new(&conn);
        let item = dao.get_by_id(payload.id)?;

        dao.delete(payload.id)?;

        Ok(ApiResult::success(format!("{} di hapus", &item.item_name)))
    }
}
