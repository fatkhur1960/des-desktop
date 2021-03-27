use super::{EntriesResult, Error as ServiceError, IdPayload, PayloadEntries};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::{crypto, dao::user_dao::UserDao, error::ErrorCode, models, ApiResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddUser {
    pub full_name: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUser {
    pub id: i32,
    pub full_name: Option<String>,
    pub username: Option<String>,
    pub active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePassword {
    pub id: i32,
    pub password: String,
}

#[derive(Default)]
pub struct UserService;

#[service]
impl UserService {
    #[route]
    pub fn add_user(payload: AddUser) -> ApiResult<String> {
        let conn = state.db();
        let dao = UserDao::new(&conn);

        let password = crypto::get_passhash(&payload.password);

        dao.add_user(&payload.full_name, &payload.username, &password)?;

        Ok(ApiResult::success(format!(
            "{} tersimpan",
            &payload.full_name
        )))
    }

    #[route]
    pub fn update_user(payload: UpdateUser) -> ApiResult<String> {
        let conn = state.db();
        let dao = UserDao::new(&conn);

        let user = dao.get_by_id(payload.id)?;

        dao.update_user(
            user.id,
            &payload.full_name.unwrap_or(user.full_name.to_owned()),
            &payload.username.unwrap_or(user.username),
            payload.active.unwrap_or(user.active),
        )?;

        Ok(ApiResult::success(format!("{} diupdate", &user.full_name)))
    }

    #[route]
    pub fn update_password(payload: UpdatePassword) -> ApiResult<String> {
        let conn = state.db();
        let dao = UserDao::new(&conn);

        let user = dao.get_by_id(payload.id)?;

        let password = crypto::get_passhash(&payload.password);

        dao.update_password(user.id, &password)?;

        Ok(ApiResult::success(format!("Password {} diupdate", &user.full_name)))
    }

    #[route]
    pub fn delete_user(payload: IdPayload) -> ApiResult<String> {
        let conn = state.db();
        let dao = UserDao::new(&conn);

        let user = dao.get_by_id(payload.id)?;

        dao.delete(user.id)?;

        Ok(ApiResult::success(format!("{} dihapus", &user.full_name)))
    }

    #[route]
    pub fn get_user(payload: IdPayload) -> ApiResult<models::User> {
        let conn = state.db();
        let dao = UserDao::new(&conn);

        let user = dao.get_by_id(payload.id)?;

        Ok(ApiResult::success(user))
    }

    #[route]
    pub fn get_users(payload: PayloadEntries) -> ApiResult<EntriesResult<models::User>> {
        let conn = state.db();
        let dao = UserDao::new(&conn);

        let (entries, count) = dao.get_users(payload.query, payload.offset, payload.limit)?;

        Ok(ApiResult::success(EntriesResult { entries, count }))
    }
}
