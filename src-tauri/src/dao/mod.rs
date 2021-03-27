use diesel::{BoxableExpression, sql_types, sqlite::Sqlite};
pub type Filterer<T> = Box<dyn BoxableExpression<T, Sqlite, SqlType = sql_types::Bool>>;

pub mod user_dao;
pub mod item_dao;
pub mod sale_dao;