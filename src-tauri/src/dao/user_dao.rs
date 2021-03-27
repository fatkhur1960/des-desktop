//! Dao implementation for User
//!

use crate::{models::User, schema::users, service::Result, sqlutil::lower};
use chrono::prelude::*;
use diesel::{prelude::*, sql_types, sqlite::Sqlite};

#[derive(Insertable)]
#[table_name = "users"]
pub struct UserNew<'a> {
    pub full_name: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub active: bool,
}

#[derive(AsChangeset)]
#[table_name = "users"]
pub struct UserUpdate<'a> {
    pub full_name: &'a str,
    pub username: &'a str,
    pub active: bool,
}

pub struct UserDao<'a> {
    db: &'a SqliteConnection,
}

impl<'a> UserDao<'a> {
    pub fn new(db: &'a SqliteConnection) -> Self {
        UserDao { db }
    }

    pub fn add_user(&self, full_name: &str, username: &str, password: &str) -> Result<usize> {
        use crate::schema::users::dsl;

        diesel::insert_into(dsl::users)
            .values(&UserNew {
                full_name,
                username,
                password,
                active: true,
            })
            .execute(self.db)
            .map_err(From::from)
    }

    pub fn update_user(
        &self,
        id: i32,
        full_name: &str,
        username: &str,
        active: bool,
    ) -> Result<usize> {
        use crate::schema::users::dsl;

        diesel::update(dsl::users.filter(dsl::id.eq(id)))
            .set(&UserUpdate {
                full_name,
                username,
                active,
            })
            .execute(self.db)
            .map_err(From::from)
    }

    pub fn get_by_username(&self, username: &str) -> Result<User> {
        use crate::schema::users::dsl;

        dsl::users
            .filter(dsl::username.eq(username))
            .first(self.db)
            .map_err(From::from)
    }

    pub fn get_by_id(&self, id: i32) -> Result<User> {
        use crate::schema::users::dsl;

        dsl::users
            .filter(dsl::id.eq(id))
            .first(self.db)
            .map_err(From::from)
    }

    /// Get users
    pub fn get_users(
        &self,
        query: Option<String>,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<User>, i64)> {
        use super::Filterer;
        use crate::schema::users::{self, dsl};

        assert!(offset > -1, "Invalid offset");
        assert!(limit > -1, "Invalid limit");
        assert!(limit < 1_000_000, "Invalid limit");

        let mut filterer: Filterer<users::table> = Box::new(dsl::id.ne(0));

        if let Some(keyword) = query {
            let like_clause = format!("%{}%", keyword.to_lowercase());
            filterer = Box::new(
                filterer.and(
                    lower(dsl::full_name)
                        .like(like_clause.to_owned())
                        .or(lower(dsl::username).like(like_clause.to_owned())),
                ),
            );
        }

        Ok((
            dsl::users
                .filter(&filterer)
                .offset(offset)
                .limit(limit)
                .order(dsl::id.desc())
                .load(self.db)?,
            dsl::users
                .filter(&filterer)
                .select(diesel::dsl::count(dsl::id))
                .first(self.db)?,
        ))
    }

    pub fn delete(&self, id: i32) -> Result<usize> {
        use crate::schema::users::dsl;

        diesel::delete(dsl::users.filter(dsl::id.eq(id)))
            .execute(self.db)
            .map_err(From::from)
    }

    pub fn update_last_login(&self, id: i32, dt: NaiveDateTime) -> Result<usize> {
        use crate::schema::users::{self, dsl};
        diesel::update(dsl::users.filter(dsl::id.eq(id)))
            .set(dsl::last_login.eq(&dt))
            .execute(self.db)
            .map_err(From::from)
    }

    pub fn update_password(&self, id: i32, password: &str) -> Result<usize> {
        use crate::schema::users::{self, dsl};
        diesel::update(dsl::users.filter(dsl::id.eq(id)))
            .set(dsl::password.eq(password))
            .execute(self.db)
            .map_err(From::from)
    }
}
