use reqwest::{Client, Method, Request, Url};
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::error::Error;
use std::fmt::{self, Display};
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
        Ok(Record {
            open: fields[3].parse()?,
            high: fields[4].parse()?,
            low: fields[5].parse()?,
            close: fields[6].parse()?,
            volume: fields[7].parse()?,
        })
    }
}

#[derive(Debug)]
pub enum TradingPair {
    BTCUSD,
}

impl TradingPair {
    pub fn get_record(&self, url: &str) -> Result<Record, Box<Error>> {
        let relative_path = format!("/{}", self);
        let url = Url::parse(url)?.join(&relative_path)?;

        let request = Request::new(Method::GET, url);
        let mut response = Client::new().execute(request)?;

        Record::from_csv_str(&response.text()?)
    }

    pub fn subscribe(&self, url: &str) -> Result<Connection, Box<Error>> {
        let relative_path = format!("/subscribe/{}", self);
        let url = Url::parse(url)?.join(&relative_path)?;

        let request = Request::new(Method::POST, url);
        let mut response = Client::new().execute(request)?;

        let ip = response.remote_addr().ok_or("no remote ip")?.ip();
        let port = response.text()?.trim().parse::<u16>()?;

        let reader = BufReader::new(TcpStream::connect((ip, port))?);
        Ok(Connection(reader))
    }
}

impl Display for TradingPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Connection(BufReader<TcpStream>);

impl Iterator for Connection {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = String::with_capacity(100);
        let _ = self.0.read_line(&mut buf).ok()?;
        Record::from_csv_str(&buf).ok()
    }
}
