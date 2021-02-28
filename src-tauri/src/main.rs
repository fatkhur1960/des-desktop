#![recursion_limit = "512"]
#![allow(unused_imports, unused_variables, dead_code, unused_macros)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate des_macros;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate diesel;
extern crate bcrypt;
extern crate chrono;

pub type ID = i64;

use std::env;

use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use tauri::Webview;
mod result;
pub use result::{ApiResult, ServiceError};
mod crypto;
mod dao;
mod db;
mod error;
mod models;
mod schema;
mod service;
mod sqlutil;
mod forecast_util;

#[macro_use]
mod macros;

use service::{
    auth::AuthService, forecast::ForecastService, item::ItemService, sales::SaleService,
    user::UserService,
};

impl_service!([
    AuthService,
    ItemService,
    UserService,
    SaleService,
    ForecastService
]);

fn main() {
    env_logger::init();
    dotenv::dotenv().ok();
    env::set_var("DATABASE_URL", "./tokoku.db");

    tauri::AppBuilder::new()
        .invoke_handler(handle_service)
        .build()
        .run();
}
