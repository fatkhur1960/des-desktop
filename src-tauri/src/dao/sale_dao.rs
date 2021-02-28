use crate::{
    models::{Item, Sale, SaleHistory, SaleItem, SaleItemHistory},
    schema::{items, sales},
    service::Result,
    sqlutil::{lower, strftime},
};
use chrono::prelude::*;
use diesel::{prelude::*, sqlite::Sqlite};

#[derive(Insertable)]
#[table_name = "sales"]
struct NewSale {
    pub item_id: i32,
    pub sale_value: i32,
    pub ts: NaiveDateTime,
}

#[derive(AsChangeset)]
#[table_name = "sales"]
struct UpdateSale {
    pub sale_value: i32,
    pub ts: NaiveDateTime,
}

/// Data Access Object for Sale
pub struct SaleDao<'a> {
    db: &'a SqliteConnection,
}

impl<'a> SaleDao<'a> {
    pub fn new(db: &'a SqliteConnection) -> Self {
        SaleDao { db }
    }

    /// Create new Sale
    pub fn create(&self, item_id: i32, sale_value: i32, ts: NaiveDateTime) -> Result<usize> {
        use crate::schema::sales::{self, dsl};

        diesel::insert_into(sales::table)
            .values(&NewSale {
                item_id,
                sale_value,
                ts,
            })
            .execute(self.db)
            .map_err(From::from)
    }

    /// Update sale
    pub fn update(&self, id: i32, sale_value: i32, ts: NaiveDateTime) -> Result<usize> {
        use crate::schema::sales::{self, dsl};
        diesel::update(dsl::sales.filter(dsl::id.eq(id)))
            .set(&UpdateSale { sale_value, ts })
            .execute(self.db)
            .map_err(From::from)
    }

    /// Delete item
    pub fn delete(&self, id: i32) -> Result<usize> {
        use crate::schema::sales::{self, dsl};

        diesel::delete(dsl::sales.filter(dsl::id.eq(id)))
            .execute(self.db)
            .map_err(From::from)
    }

    /// Mendapatkan item berdasarkan id-nya.
    pub fn get_by_id(&self, id: i32) -> Result<SaleItem> {
        use crate::schema::items::{self, dsl as dsl_item};
        use crate::schema::sales::{self, dsl};
        dsl::sales
            .inner_join(items::table.on(dsl::item_id.eq(dsl_item::id)))
            .select((
                dsl::id,
                dsl::item_id,
                dsl_item::item_name,
                dsl::sale_value,
                dsl::ts,
            ))
            .filter(dsl::id.eq(id))
            .first(self.db)
            .map_err(From::from)
    }

    pub fn get_sales_by_item(&self, id: i32) -> Result<(Vec<SaleHistory>, i64)> {
        use super::Filterer;
        use crate::schema::items::{self, dsl as dsl_item};
        use crate::schema::sales::{self, dsl};

        let raw_query = sales::table.inner_join(items::table.on(dsl::item_id.eq(dsl_item::id)));
        let filterer: Filterer<_> = Box::new(dsl::item_id.eq(id));

        Ok((
            raw_query
                .select((
                    dsl::id,
                    dsl::item_id,
                    dsl::sale_value,
                    strftime("%m", dsl::ts),
                    strftime("%Y", dsl::ts),
                ))
                .filter(&filterer)
                .order(dsl::ts.asc())
                .load(self.db)?,
            raw_query
                .filter(&filterer)
                .select(diesel::dsl::count(dsl::id))
                .first(self.db)?,
        ))
    }

    /// Get stock histories based on Item
    pub fn get_sales(
        &self,
        filter: crate::service::QueryFilter,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<SaleItem>, i64)> {
        use super::Filterer;
        use crate::schema::items::{self, dsl as dsl_item};
        use crate::schema::sales::{self, dsl};

        assert!(offset > -1, "Invalid offset");
        assert!(limit > -1, "Invalid limit");
        assert!(limit < 1_000_000, "Invalid limit");

        let raw_query = sales::table.inner_join(items::table.on(dsl::item_id.eq(dsl_item::id)));

        let mut filterer: Filterer<_> = Box::new(dsl::id.ne(0));

        if let Some(keyword) = filter.query {
            let like_clause = format!("%{}%", keyword.to_lowercase());
            filterer = Box::new(filterer.and(dsl_item::item_name.like(like_clause)));
        }

        if let Some(item_id) = filter.id {
            filterer = Box::new(filterer.and(dsl::item_id.eq(item_id)));
        }

        if let Some(month) = filter.month {
            filterer = Box::new(filterer.and(strftime("%m", dsl::ts).eq(month)));
        }

        if let Some(year) = filter.year {
            filterer = Box::new(filterer.and(strftime("%Y", dsl::ts).eq(year)));
        }

        Ok((
            raw_query
                .select((
                    dsl::id,
                    dsl::item_id,
                    dsl_item::item_name,
                    dsl::sale_value,
                    dsl::ts,
                ))
                .filter(&filterer)
                .offset(offset)
                .limit(limit)
                .order(dsl::ts.desc())
                .load(self.db)?,
            raw_query
                .filter(&filterer)
                .select(diesel::dsl::count(dsl::id))
                .first(self.db)?,
        ))
    }

    /// Get stock histories based on Item
    pub fn get_sale_histories(
        &self,
        filter: crate::service::QueryFilter,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<SaleItemHistory>, i64)> {
        use super::Filterer;
        use crate::schema::items::{self, dsl as dsl_item};
        use crate::schema::sales::{self, dsl};

        assert!(offset > -1, "Invalid offset");
        assert!(limit > -1, "Invalid limit");
        assert!(limit < 1_000_000, "Invalid limit");

        let mut filterer: Filterer<items::table> = Box::new(dsl_item::id.ne(0));

        if let Some(keyword) = filter.query {
            let like_clause = format!("%{}%", keyword.to_lowercase());
            filterer = Box::new(filterer.and(dsl_item::item_name.like(like_clause)));
        }

        if let Some(item_id) = filter.id {
            filterer = Box::new(filterer.and(dsl_item::id.eq(item_id)));
        }

        let raw_item = dsl_item::items
            .filter(&filterer)
            .offset(offset)
            .limit(limit)
            .order(dsl_item::id.desc())
            .load::<Item>(self.db)?;
        let items: Vec<SaleItemHistory> = raw_item
            .iter()
            .map(|item| {
                let sales = SaleHistory::belonging_to(item)
                    .select((
                        dsl::id,
                        dsl::item_id,
                        dsl::sale_value,
                        strftime("%m", dsl::ts),
                        strftime("%Y", dsl::ts),
                    ))
                    .limit(12)
                    .offset(0)
                    .order(dsl::id.desc())
                    .order(dsl::ts.asc())
                    .load::<SaleHistory>(self.db)
                    .unwrap_or(Vec::new());

                SaleItemHistory {
                    id: item.id,
                    item_name: item.item_name.to_owned(),
                    sales,
                }
            })
            .collect();

        Ok((
            items,
            dsl_item::items
                .filter(&filterer)
                .select(diesel::dsl::count(dsl_item::id))
                .first(self.db)?,
        ))
    }
}
