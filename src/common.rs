extern crate serde;
extern crate serde_json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Record {
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    pub volume: f64,
}

pub fn execute() {
    println!("common Hello World!");
}