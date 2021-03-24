//! Models definitions
use crate::schema::{items, sales};
use crate::ID;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[doc(hidden)]
#[derive(Queryable, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub full_name: String,
    pub active: bool,
    pub last_login: NaiveDateTime,
}

#[doc(hidden)]
#[derive(Queryable, Serialize)]
pub struct Sale {
    pub id: i32,
    pub item_id: i32,
    pub sale_value: i32,
    pub ts: NaiveDateTime,
}

#[doc(hidden)]
#[derive(Debug, Queryable, Serialize)]
pub struct SaleItem {
    pub id: i32,
    pub item_id: i32,
    pub item_name: String,
    pub sale_value: i32,
    pub ts: NaiveDateTime,
}

#[doc(hidden)]
#[derive(Identifiable, Associations, PartialEq, Debug, Queryable, Serialize, Clone)]
#[table_name = "items"]
pub struct Item {
    pub id: i32,
    pub item_name: String,
    pub item_desc: String,
}

#[doc(hidden)]
#[derive(Identifiable, Associations, PartialEq, Deserialize, Debug, Queryable, Serialize, Clone)]
#[belongs_to(Item)]
#[table_name = "sales"]
pub struct SaleHistory {
    pub id: i32,
    pub item_id: i32,
    pub sale_value: i32,
    pub month: String,
    pub year: String,
}

#[doc(hidden)]
#[derive(Debug, Queryable, Serialize, Clone)]
pub struct SaleItemHistory {
    pub id: i32,
    pub item_name: String,
    pub sales: Vec<SaleHistory>,
}

#[doc(hidden)]
#[derive(Debug, Deserialize, Serialize, Clone, Eq, Ord, PartialOrd, PartialEq)]
pub struct ForecastHistory {
    pub sale_value: i32,
    pub date: NaiveDate,
}

#[doc(hidden)]
#[derive(Debug, Deserialize, Queryable, Serialize, Clone)]
pub struct ForecastResult {
    pub real: Vec<ForecastHistory>,
    pub forecast: Vec<ForecastHistory>,
    pub alpha: f64,
}

