use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResult<T> {
  error: bool,
  message: String,
  pub data: Option<T>,
}

impl<T: Serialize> ApiResult<T> {
  pub fn new(error: bool, message: String, data: Option<T>) -> Self {
    ApiResult {
      error,
      message,
      data,
    }
  }

  pub fn success(data: T) -> Self {
    Self::new(false, "".to_owned(), Some(data))
  }
}

impl ApiResult<()> {
  /// Buat hasil error
  pub fn error(message: String) -> ApiResult<()> {
    ApiResult {
      error: true,
      message,
      data: None,
    }
  }
}

impl<'a> std::fmt::Display for ApiResult<()> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl<'a> std::error::Error for ApiResult<()> {}

// An error type we define
// We could also use the `anyhow` lib here
#[derive(Debug, Clone)]
pub struct ServiceError {
  pub message: String,
}

impl ServiceError {
  pub fn new(message: String) -> Self {
    Self { message }
  }
}

impl std::fmt::Display for ServiceError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

// Tauri uses the `anyhow` lib so custom error types must implement std::error::Error
// and the function call should call `.into()` on it
impl std::error::Error for ServiceError {}
