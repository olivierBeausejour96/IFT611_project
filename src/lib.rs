pub mod client;
pub mod logger;
pub mod server;

use reqwest::{Client, Method, Request, Url};
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::error::Error;
use std::io::{BufRead, BufReader};
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

    pub fn from_csv_str(csv_str: &str) -> Result<Self, Box<Error>> {
        let fields: Vec<_> = csv_str.split(',').map(str::trim).collect();
        Ok(
            Record {
                open: fields[3].parse()?,
                high: fields[4].parse()?,
                low: fields[5].parse()?,
                close: fields[6].parse()?,
                volume: fields[7].parse()?,
            }
        )
    }
}

pub fn get_btc_record(url: &str) -> Result<Record, Box<Error>> {
    let request = Request::new(Method::GET, Url::parse(url)?.join("/BTCUSD")?);
    let mut response = Client::new().execute(request)?;
    Record::from_csv_str(&response.text()?)
}

pub fn subscribe_btc(url: &str) -> Result<BufReader<TcpStream>, Box<Error>> {
    let request = Request::new(Method::POST, Url::parse(url)?.join("/subscribe/BTCUSD")?);
    let mut response = Client::new().execute(request)?;
    let ip = response.remote_addr().ok_or("no remote ip")?.ip();
    let port = response.text()?.trim().parse::<u16>()?;

    let reader = BufReader::new(TcpStream::connect((ip, port))?);
    Ok(reader)
}

pub fn read_record(reader: &mut BufReader<TcpStream>) -> Result<Record, Box<Error>> {
    let mut buf = String::with_capacity(100);
    let _ = reader.read_line(&mut buf)?;
    Record::from_csv_str(&buf)
}
