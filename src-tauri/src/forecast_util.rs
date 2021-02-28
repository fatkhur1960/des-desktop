use std::{collections::HashMap, ops::Index};

use crate::models::SaleHistory;

pub struct Forecast {
    ds: Vec<SaleHistory>,
}

impl Forecast {
    pub fn new(ds: Vec<SaleHistory>) -> Self {
        Forecast { ds }
    }

    pub fn forecast(&mut self, alpha: Option<f64>, next: i32, test: bool) -> Vec<f64> {
        let alpha = alpha.unwrap_or(0.1);
        let ds_len = self.ds.len();
        let ds = self.ds.clone();

        // Mencari moving average (MA)
        let mut ma: Vec<f64> = Vec::new();
        for i in 0..ds_len {
            if i < ds_len - 2 {
                let ma_tmp: f64 = (ds[i].sale_value as f64 + ds[i + 1].sale_value as f64) / 2.0;
                ma.push(ma_tmp);
            }
        }

        // Mencari S't
        let mut seg_1: Vec<f64> = Vec::new();
        for i in 0..ds_len {
            if i < ds_len - 3 {
                let tmp = (alpha * (ds[i + 3].sale_value as f64)) + (1.0 - alpha) * ma[i];
                seg_1.push(tmp);
            }
        }

        // Mencari S''t
        let mut seg_2: Vec<f64> = Vec::new();
        for i in 0..ma.len() {
            if i < ma.len() - 2 {
                let tmp = (alpha * ma[i + 2]) + (1.0 - alpha) * seg_1[i];
                seg_2.push(tmp);
            }
        }

        // Mencari scope bt
        let bt: f64 = (alpha / (1.0 - alpha)) * (seg_1[0] - seg_2[0]);

        // Mencari scope st
        let st: f64 = (2.0 * seg_1[0]) - seg_2[0];

        let offset = if !test {
            (ds_len as i32) - 4 + next
        } else {
            (ds_len as i32) - 6
        };

        // Mencari Ft+m
        let mut ft: Vec<f64> = Vec::new();
        for i in 0..next {
            let m = (offset + i) as f64;
            let tmp = st + (bt * m);
            ft.push(tmp);
        }

        ft
    }

    pub fn get_optimized_setting(&mut self) -> (f64, f64) {
        let ds_len = self.ds.len();
        let next = (ds_len - 5) as i32;
        let new_ds: Vec<f64> = self.ds[5..ds_len]
            .into_iter()
            .map(|d| d.sale_value as f64)
            .collect();

        let mut alpha_tmp = Vec::new();
        let mut mse_tmp: Vec<f64> = Vec::new();
        for i in 1..10 {
            let alpha = (i as f64) / 10.0;
            let ft = self.forecast(Some(alpha), next, true);

            let mut at_ft = 0 as f64;
            for i in 0..new_ds.len() {
                at_ft += (new_ds[i] - ft[i]).powf(2.0);
            }
            let mse = at_ft / new_ds.len() as f64;

            alpha_tmp.push(alpha);
            mse_tmp.push(mse);
        }

        let mse = mse_tmp
            .clone()
            .into_iter()
            .fold(f64::INFINITY, |a, b| a.min(b));
        let index = mse_tmp.iter().position(|r| *r == mse).unwrap();
        let alpha = alpha_tmp[index].clone();

        (mse.round(), alpha)
    }
}
