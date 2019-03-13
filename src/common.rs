extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Record {
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    pub volume: f64,
}

impl Default for Record {
    fn default() -> Record {
        Record {
            open: 32.0,
            high: 32.0,
            low: 32.0,
            close: 32.0,
            volume: 64.0,
        }
    }
}

impl Record {
    pub fn to_csv_str(&self) -> String {
        format!(
            "{},{},{},{},{}",
            self.open, self.high, self.low, self.close, self.volume
        )
    }
}

pub fn execute() {
    println!("common Hello World!");
}
