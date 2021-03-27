use super::{
    EntriesResult, Error as ServiceError, FcPayload, IdPayload, PayloadEntries, QueryFilter,
};
use chrono::{NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    dao::{item_dao::ItemDao, sale_dao::SaleDao},
    error::ErrorCode,
    forecast_util::Forecast,
    models::{self, ForecastHistory, ForecastResult, SaleHistory, SaleItem},
    ApiResult,
};

#[derive(Default)]
pub struct ForecastService;

#[service]
impl ForecastService {
    #[route]
    pub fn predict(payload: FcPayload) -> ApiResult<ForecastResult> {
        let conn = state.db();
        let sale_dao = SaleDao::new(&conn);
        let item_dao = ItemDao::new(&conn);

        let item = item_dao.get_by_id(payload.id)?;
        let latest = sale_dao.get_latest_sale(item.id)?;
        let (ds, _) = sale_dao.get_sales_by_item(item.id)?;

        let mut forecast = Forecast::new(ds.clone());
        let (_, alpha) = forecast.get_optimized_setting();
        let res = forecast.forecast(Some(alpha), payload.next, false);

        let mut reset_month = 0;
        let mut round = 0;

        let tmp_month = latest.month.parse::<i32>().expect("cannot parse month");
        let tmp_year = latest.year.parse::<i32>().expect("cannot parse year");

        let mut real: Vec<ForecastHistory> = ds
            .into_iter()
            .map(|a| {
                let month = a.month.parse::<u32>().unwrap();
                let year = a.year.parse::<i32>().unwrap();

                ForecastHistory {
                    sale_value: a.sale_value,
                    date: NaiveDate::from_ymd(year, month, 10),
                }
            })
            .collect();

        let mut forecast: Vec<ForecastHistory> = res
            .into_iter()
            .enumerate()
            .map(|(index, f)| {
                let mut m = tmp_month;

                let month = {
                    m += (index + 1) as i32;
                    if m == 13 {
                        round += 1;
                    }
                    if m > 12 {
                        m = 0;
                        reset_month += 1;
                        m += reset_month;
                    }
                    if reset_month == 12 {
                        round += 1;
                        reset_month = 0;
                    }

                    m
                };
                let year = tmp_year + round;

                ForecastHistory {
                    sale_value: f.round() as i32,
                    date: NaiveDate::from_ymd(year, month as u32, 10),
                }
            })
            .collect();
        forecast.extend(real.clone());
        forecast.sort_by_key(|a| a.date);
        real.sort_by_key(|a| a.date);

        let result = ForecastResult {
            real,
            forecast,
            alpha,
        };

        Ok(ApiResult::success(result))
    }
}
