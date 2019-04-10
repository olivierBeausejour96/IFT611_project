pub mod client;
pub mod logger;
pub mod server;

use reqwest::{Client, Method, Request, Url};
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::error::Error;
use std::net::TcpStream;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
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

pub fn get_btc_record(url: &str) -> Result<Record, Box<Error>> {
    let request = Request::new(Method::GET, Url::parse(url)?.join("/BTCUSD")?);
    let mut response = Client::new().execute(request)?;
    let record: Record = response.json()?;
    Ok(record)
}

pub fn subscribe_btc(url: &str) -> Result<TcpStream, Box<Error>> {
    let request = Request::new(Method::GET, Url::parse(url)?.join("/subscribe/BTCUSD")?);
    let mut response = Client::new().execute(request)?;
    let ip = response.remote_addr().ok_or("no remote ip")?.ip();
    let port = response.text()?.parse::<u16>()?;

    let stream = TcpStream::connect((ip, port))?;
    Ok(stream)
}
