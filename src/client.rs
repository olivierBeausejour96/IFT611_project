use crate::logger::Context;
use circular_queue::CircularQueue;
use reqwest::{Client, Method, Request, Url};
use serde::{Deserialize, Serialize};
use std::convert::From;
use std::default::Default;
use std::error::Error;
use std::fmt::{self, Display};
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

#[derive(Debug)]
pub enum DecisionLogs {
    Buy,
    Sell,
    Wait,
}

impl Context for DecisionLogs {
    fn context_string(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Debug)]
pub enum TradingStrategy {
    Dummy,
}

impl From<&str> for TradingStrategy {
    fn from(s: &str) -> Self {
        match s {
            "dummy" => TradingStrategy::Dummy,
            _ => panic!("invalid strategy name: {}", s),
        }
    }
}

impl TradingStrategy {
    pub fn make_decision(&self, queue: &CircularQueue<Record>) -> DecisionLogs {
        match self {
            TradingStrategy::Dummy => Self::dummy(queue),
        }
    }

    fn dummy(queue: &CircularQueue<Record>) -> DecisionLogs {
        static TRESHOLD: f32 = 5.0;
        let diff = queue.iter().nth(0).unwrap().close - queue.iter().last().unwrap().close;

        if diff > TRESHOLD {
            DecisionLogs::Buy
        } else if diff < -TRESHOLD {
            DecisionLogs::Sell
        } else {
            DecisionLogs::Wait
        }
    }
}

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

    pub fn subscribe(&self, url: &str) -> Result<Connection<BufReader<TcpStream>>, Box<Error>> {
        let relative_path = format!("/subscribe/{}", self);
        let url = Url::parse(url)?.join(&relative_path)?;

        let request = Request::new(Method::POST, url);
        let mut response = Client::new().execute(request)?;

        let ip = response.remote_addr().ok_or("no remote ip")?.ip();
        let port = response.text()?.trim().parse::<u16>()?;

        let reader = BufReader::new(TcpStream::connect((ip, port))?);
        Ok(Connection::new(reader))
    }
}

impl Display for TradingPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Connection<T> {
    reader: T,
    buffer: String,
}

impl<T> Connection<T> {
    fn new(reader: T) -> Self {
        Self {
            reader,
            buffer: String::with_capacity(200),
        }
    }
}

impl<T: BufRead> Iterator for Connection<T> {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        let _ = self.reader.read_line(&mut self.buffer).ok()?;
        Record::from_csv_str(&self.buffer).ok()
    }
}
