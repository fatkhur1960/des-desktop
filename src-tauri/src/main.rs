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
use std::{env, io::Write};
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


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env::set_var("DATABASE_URL", "./tokoku.db");

    if !std::path::Path::new("./tokoku.db").exists() {
        let target = "https://github.com/fatkhur1960/des-desktop/raw/master/tokoku.db";
        let response = reqwest::get(target).await?;

        let mut file = std::fs::File::create("./tokoku.db")?;
        let content =  response.bytes().await?;
        let mut pos = 0;
        while pos < content.len() {
            let bytes_written = file.write(&content[pos..])?;
            pos += bytes_written;
        }
    }

    tauri::AppBuilder::new()
        .invoke_handler(handle_service)
        .build()
        .run();

    Ok(())
}
