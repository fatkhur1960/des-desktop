use super::Error as ServiceError;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::{crypto, dao::user_dao::UserDao, error::ErrorCode, ApiResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthPayload {
    pub username: String,
    pub password: String,
}

#[derive(Default)]
pub struct AuthService;

#[service]
impl AuthService {
    #[route]
    pub fn authorize(payload: AuthPayload) -> ApiResult<String> {
        let conn = state.db();
        let dao = UserDao::new(&conn);
        let user = match dao.get_by_username(&payload.username) {
            Ok(user) => user,
            Err(_) => {
                return Err(ServiceError::InvalidParameter(
                    ErrorCode::Unauthorized as i32,
                    "Username tidak ditemukan".to_string(),
                ))
            }
        };

        if !user.active {
            return Err(ServiceError::InvalidParameter(
                ErrorCode::Unauthorized as i32,
                "Akun diblokir".to_string(),
            ));
        }

        if !crypto::password_match(&payload.password, &user.password) {
            return Err(ServiceError::InvalidParameter(
                ErrorCode::Unauthorized as i32,
                "Password tidak cocok".to_string(),
            ));
        }

        dao.update_last_login(user.id, Utc::now().naive_local())?;

        Ok(ApiResult::success(user.full_name))
    }
}
