use std::env;

use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use diesel::Connection;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
pub type DbConn = r2d2::PooledConnection<ConnectionManager<SqliteConnection>>;

lazy_static! {
    pub static ref DB_CONN_POOL: Pool = {
        let conn_man = ConnectionManager::new(
            env::var("DATABASE_URL").expect("no DATABASE_URL env var"),
        );
        Pool::builder()
            .build(conn_man)
            .expect("Cannot build DB connection poll")
    };
}

pub fn connect(db_url: &str) -> SqliteConnection {
    SqliteConnection::establish(db_url).unwrap_or_else(|_| panic!("Cannot connect to `{}`", db_url))
}

pub fn clone() -> Pool {
    DB_CONN_POOL.clone()
}