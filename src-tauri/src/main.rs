#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![recursion_limit = "512"]
#![allow(unused_imports, unused_variables, dead_code, unused_macros)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate des_macros;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate diesel;
extern crate bcrypt;
extern crate chrono;

pub type ID = i64;

use models::ForecastResult;
use serde::{Deserialize, Serialize};
use std::env;
use tauri::Webview;
mod result;
pub use result::{ApiResult, ServiceError};
mod crypto;
mod dao;
mod db;
mod error;
mod forecast_util;
mod models;
mod schema;
mod service;
mod sqlutil;

#[macro_use]
mod macros;

use service::{
    auth::AuthService, forecast::ForecastService, item::ItemService, sales::SaleService,
    user::UserService, FcPayload,
};

impl_service!([
    AuthService,
    ItemService,
    UserService,
    SaleService,
    ForecastService
]);

fn main() {
    dotenv::dotenv().ok();
    env::set_var("DATABASE_URL", "./tokoku.db");

    // let app_state = crate::service::AppState::new();
    // let fs = ForecastService::wire(
    //     &app_state,
    //     "predict".to_string(),
    //     serde_json::to_value(FcPayload { id: 27, next: 24 }).unwrap(),
    // )
    // .unwrap();

    // let res: ApiResult<ForecastResult> = serde_json::from_value(fs).unwrap();

    // let output = res
    //     .data
    //     .unwrap()
    //     .forecast
    //     .into_iter()
    //     .map(|a| format!("{} - {}", a.month, a.year))
    //     .collect::<Vec<String>>();

    tauri::AppBuilder::new()
        .invoke_handler(handle_service)
        .build()
        .run();
}
