use super::{EntriesResult, Error as ServiceError, IdPayload, PayloadEntries, QueryFilter};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    dao::{item_dao::ItemDao, sale_dao::SaleDao},
    error::ErrorCode,
    models::{self, SaleItem},
    ApiResult,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddSale {
    pub item_id: i32,
    pub sale_value: i32,
    pub ts: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSale {
    pub id: i32,
    pub sale_value: Option<i32>,
    pub ts: Option<NaiveDateTime>,
}

#[derive(Default)]
pub struct SaleService;

#[service]
impl SaleService {
    #[route]
    pub fn add_sale(payload: AddSale) -> ApiResult<String> {
        let conn = state.db();
        let sale_dao = SaleDao::new(&conn);
        let item_dao = ItemDao::new(&conn);

        let item = item_dao.get_by_id(payload.item_id)?;
        sale_dao.create(item.id, payload.sale_value, payload.ts)?;

        Ok(ApiResult::success(format!(
            "Penjualan {} disimpan",
            &item.item_name
        )))
    }

    #[route]
    pub fn update_sale(payload: UpdateSale) -> ApiResult<String> {
        let conn = state.db();
        let dao = SaleDao::new(&conn);
        let sale = dao.get_by_id(payload.id)?;

        dao.update(
            payload.id,
            payload.sale_value.unwrap_or(sale.sale_value),
            payload.ts.unwrap_or(sale.ts),
        )?;

        let item_dao = ItemDao::new(&conn);
        let item = item_dao.get_by_id(sale.item_id)?;

        Ok(ApiResult::success(format!(
            "Penjualan {} diupdate",
            &item.item_name.clone()
        )))
    }

    #[route]
    pub fn get_sales(payload: PayloadEntries) -> ApiResult<EntriesResult<models::SaleItem>> {
        let conn = state.db();
        let dao = SaleDao::new(&conn);

        let filter = QueryFilter {
            id: payload.id,
            query: payload.query,
            month: payload.month,
            year: payload.year,
        };

        let (entries, count) = dao.get_sales(filter, payload.offset, payload.limit)?;

        Ok(ApiResult::success(EntriesResult { entries, count }))
    }

    #[route]
    pub fn get_sale_histories(payload: PayloadEntries) -> ApiResult<EntriesResult<models::SaleItemHistory>> {
        let conn = state.db();
        let dao = SaleDao::new(&conn);

        let filter = QueryFilter {
            id: payload.id,
            query: payload.query,
            ..Default::default()
        };

        let (entries, count) = dao.get_sale_histories(filter, payload.offset, payload.limit)?;
        Ok(ApiResult::success(EntriesResult { entries, count }))
    }

    #[route]
    pub fn get_sale(payload: IdPayload) -> ApiResult<models::SaleItem> {
        let conn = state.db();
        let dao = SaleDao::new(&conn);

        let item = dao.get_by_id(payload.id)?;

        Ok(ApiResult::success(item))
    }

    #[route]
    pub fn delete_sale(payload: IdPayload) -> ApiResult<String> {
        let conn = state.db();
        let dao = SaleDao::new(&conn);
        let sale = dao.get_by_id(payload.id)?;

        dao.delete(sale.id)?;

        let item_dao = ItemDao::new(&conn);
        let item = item_dao.get_by_id(sale.item_id)?;

        Ok(ApiResult::success(format!("{} di hapus", &item.item_name)))
    }
}
