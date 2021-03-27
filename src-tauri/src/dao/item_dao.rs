//! Dao implementation for item
//!

use crate::{models::Item, schema::items, service::Result, sqlutil::lower};
use chrono::prelude::*;
use diesel::{prelude::*, sql_types, sqlite::Sqlite};

#[derive(Insertable)]
#[table_name = "items"]
struct NewItem<'a> {
    pub item_name: &'a str,
    pub item_desc: &'a str,
}

#[derive(AsChangeset)]
#[table_name = "items"]
struct UpdateItem<'a> {
    pub item_name: &'a str,
    pub item_desc: &'a str,
}

/// Data Access Object for Item
pub struct ItemDao<'a> {
    db: &'a SqliteConnection,
}

impl<'a> ItemDao<'a> {
    pub fn new(db: &'a SqliteConnection) -> Self {
        ItemDao { db }
    }

    /// Create new Item
    pub fn create(&self, item_name: &'a str, item_desc: &'a str) -> Result<usize> {
        use crate::schema::items::{self, dsl};

        diesel::insert_into(items::table)
            .values(&NewItem {
                item_name,
                item_desc,
            })
            .execute(self.db)
            .map_err(From::from)
    }

    /// Mendapatkan item berdasarkan id-nya.
    pub fn get_by_id(&self, id: i32) -> Result<Item> {
        use crate::schema::items::{self, dsl};
        dsl::items
            .filter(dsl::id.eq(id))
            .first(self.db)
            .map_err(From::from)
    }

    /// update Item
    pub fn update(&self, id: i32, item_name: &'a str, item_desc: &'a str) -> Result<usize> {
        use crate::schema::items::{self, dsl};

        diesel::update(dsl::items.filter(dsl::id.eq(id)))
            .set(&UpdateItem {
                item_name,
                item_desc,
            })
            .execute(self.db)
            .map_err(From::from)
    }

    /// Delete item
    pub fn delete(&self, id: i32) -> Result<usize> {
        use crate::schema::items::{self, dsl};

        diesel::delete(dsl::items.filter(dsl::id.eq(id)))
            .execute(self.db)
            .map_err(From::from)
    }

    /// Get stock histories based on Item
    pub fn get_items(
        &self,
        query: Option<String>,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<Item>, i64)> {
        use crate::schema::items::{self, dsl};
        use super::Filterer;

        assert!(offset > -1, "Invalid offset");
        assert!(limit > -1, "Invalid limit");
        assert!(limit < 1_000_000, "Invalid limit");

        let mut filterer: Filterer<items::table> = Box::new(dsl::id.ne(0));

        if let Some(keyword) = query {
            let like_clause = format!("%{}%", keyword.to_lowercase());
            filterer = Box::new(
                filterer.and(
                    lower(dsl::item_name)
                        .like(like_clause.to_owned())
                        .or(lower(dsl::item_desc).like(like_clause.to_owned())),
                ),
            );
        }

        Ok((
            dsl::items
                .filter(&filterer)
                .offset(offset)
                .limit(limit)
                .order(dsl::id.desc())
                .load(self.db)?,
            dsl::items
                .filter(&filterer)
                .select(diesel::dsl::count(dsl::id))
                .first(self.db)?,
        ))
    }
}
