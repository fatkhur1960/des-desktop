pub mod auth;
pub mod item;
pub mod sales;
pub mod user;
pub mod forecast;

pub mod error;
pub type Error = error::Error;
pub type Result<I> = ::std::result::Result<I, error::Error>;
pub type WrapApiResult<I> = crate::ApiResult<I>;

use crate::db::{self, DbConn, Pool};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IdPayload {
    id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct FcPayload {
    pub id: i32,
    pub next: i32,
}

#[derive(Serialize, Deserialize)]
pub struct EntriesResult<T> {
    pub entries: Vec<T>,
    pub count: i64,
}

#[derive(Serialize, Deserialize)]
pub struct PayloadEntries {
    pub limit: i64,
    pub offset: i64,
    pub query: Option<String>,
    pub id: Option<i32>,
    pub month: Option<String>,
    pub year: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct QueryFilter {
    pub id: Option<i32>,
    pub month: Option<String>,
    pub year: Option<String>,
    pub query: Option<String>,
}

impl QueryFilter {
    pub fn new() -> Self {
        Default::default()
    }
}

/// State/context yang akan selalu bisa diakses dari handler
/// state ini berisi beberapa object yang mungkin sering digunakan
/// seperti DB connection.
#[derive(Clone)]
pub struct AppState {
    db: Pool,
}

impl AppState {
    #[doc(hidden)]
    pub fn new() -> AppState {
        AppState { db: db::clone() }
    }

    /// Get Backend DB connection
    pub fn db(&self) -> DbConn {
        self.db
            .get()
            .expect("cannot get DB connection from the r2d2 pool")
    }
}
