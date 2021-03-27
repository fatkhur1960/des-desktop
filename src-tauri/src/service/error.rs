use failure;
use std::io;
use crate::{ApiResult, error::Error as ServiceError, error::ErrorCode};

/// List of possible API errors.
#[derive(Fail, Debug)]
pub enum Error {
    /// Input/output error. This type includes errors related to files that are not
    /// a part of the Exonum storage.
    #[fail(display = "IO error: {}", _0)]
    Io(#[cause] io::Error),

    /// Bad request. This error occurs when the request contains invalid syntax.
    #[fail(display = "Bad request: {}", _1)]
    BadRequest(i32, String),

    /// Not found. This error occurs when the server cannot locate the requested
    /// resource.
    #[fail(display = "Not found: {}", _1)]
    NotFound(i32, String),

    /// Internal server error. This type can return any internal server error to the user.
    #[fail(display = "Internal server error: {}", _1)]
    InternalError(i32, #[cause] failure::Error),

    /// Error yang muncul apabila user menginputkan parameter yang tidak sesuai
    #[fail(display = "{}", _1)]
    InvalidParameter(i32, String),

    /// Error yang muncul ketika sebuah object unik telah ada
    /// biasanya dimunculkan oleh operasi creation.
    #[fail(display = "Already exists")]
    AlreadyExists,

    /// Error yang muncul ketika suatu object telah habis masa berlakunya
    /// pada saat transaksi misalnya.
    #[fail(display = "{} expired", _0)]
    Expired(&'static str),

    /// Error yang bisa digunakan untuk menampilkan kode dan deskripsi secara custom.
    #[fail(display = "error code {}: {}", _0, _1)]
    CustomError(i32, String),

    /// Unauthorized error. This error occurs when the request lacks valid
    /// authentication credentials.
    #[fail(display = "Unauthorized")]
    Unauthorized,

	#[fail(display = "Storage error: {}", _0)]
    Storage(#[cause] diesel::result::Error),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<failure::Error> for Error {
    fn from(e: failure::Error) -> Self {
        Error::InternalError(504, e)
    }
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        Error::Storage(e)
    }
}