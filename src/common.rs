extern crate serde;
extern crate serde_json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Record {
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    pub volume: f64,
}

pub trait Default<T> {
    fn default() -> T;
}

impl Default<Record> for Record {
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

pub fn execute() {
    println!("common Hello World!");
}
