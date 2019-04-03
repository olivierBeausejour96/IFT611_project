use crate::common::Record;

use std::io::Write;
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use tiny_http::{Method, Request, Response, Server};

#[derive(Serialize, Deserialize, Debug)]
struct CSVRecord {
    #[serde(rename = "Unix Timestamp")]
    timestamp: String,
    #[serde(rename = "Date")]
    date: String,
    #[serde(rename = "Symbol")]
    symbol: String,
    #[serde(rename = "Open")]
    open: f32,
    #[serde(rename = "High")]
    high: f32,
    #[serde(rename = "Low")]
    low: f32,
    #[serde(rename = "Close")]
    close: f32,
    #[serde(rename = "Volume")]
    volume: f64,
}

impl From<CSVRecord> for Record {
    fn from(rec: CSVRecord) -> Self {
        Record {
            open: rec.open,
            high: rec.high,
            low: rec.low,
            close: rec.close,
            volume: rec.volume,
        }
    }
}

const PERIOD: u64 = 1000;
const PERIOD_DURATION: Duration = Duration::from_micros(PERIOD);

pub fn execute(file: &str, http_port: u16) {
    let records = Arc::new(load_data(file));
    let streams = Arc::new(Mutex::new(vec![]));

    let server = Server::http((Ipv4Addr::LOCALHOST, http_port)).unwrap();
    let start_time = Instant::now();

    {
        let streams = streams.clone();
        let start_time = start_time;
        let records = records.clone();
        thread::spawn(move || {
            periodic_push(&streams, start_time, &records);
        });
    }

    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
    let push_port = listener.local_addr().unwrap().port();
    {
        let streams = streams.clone();
        thread::spawn(move || {
            for stream in listener.incoming() {
                if let Ok(stream) = stream {
                    streams.lock().unwrap().push(stream);
                }
            }
        });
    }

    for request in server.incoming_requests() {
        handle_request(request, start_time, &records, push_port);
    }
}

fn periodic_push(streams: &Mutex<Vec<TcpStream>>, start_time: Instant, data: &[Record]) {
    let mut should_wake = Instant::now() + PERIOD_DURATION;
    loop {
        let start = Instant::now();
        push_data(streams, start_time, data);
        let wait_time = PERIOD_DURATION - (should_wake - start) - start.elapsed();
        should_wake = start + wait_time;
        thread::sleep(wait_time);
    }
}

fn push_data(streams: &Mutex<Vec<TcpStream>>, start_time: Instant, data: &[Record]) {
    let mut streams = streams.lock().unwrap();
    let mut to_remove = vec![];
    let current_data = get_current_data(start_time, data).into_bytes();
    for (i, stream) in streams.iter_mut().enumerate() {
        let result = stream.write_all(&current_data);
        if result.is_err() {
            to_remove.push(i);
        }
    }
    for i in to_remove {
        streams.remove(i);
    }
}

fn load_data(filename: &str) -> Vec<Record> {
    let mut reader = csv::Reader::from_path(filename).unwrap();

    reader
        .deserialize::<CSVRecord>()
        .map(|result| Record::from(result.unwrap()))
        .collect()
}

fn handle_request(req: Request, start_time: Instant, btc_records: &[Record], push_port: u16) {
    let response = match (req.method(), req.url()) {
        (&Method::Get, "/") => Response::from_string("Hello!"),
        (&Method::Get, "/BTCUSD") => {
            Response::from_string(get_current_data(start_time, btc_records))
        }
        (&Method::Post, "/subscribe/BTCUSD") => Response::from_string(push_port.to_string()),
        _ => Response::from_string("").with_status_code(404),
    };
    req.respond(response).unwrap();
}

fn get_current_data(start_time: Instant, records: &[Record]) -> String {
    let i = (start_time.elapsed().as_micros() / u128::from(PERIOD)) as usize;
    serde_json::to_string(&records[i]).unwrap()
}

mod test {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use crate::common::get_btc_record;
    #[allow(unused_imports)]
    use std::thread;

    #[test]
    fn deserialize_test() {
        let data = r#"Unix Timestamp,Date,Symbol,Open,High,Low,Close,Volume
1546300740000,2018-12-31 23:59:00,BTCUSD,3686.38,3692.35,3685.7,3692.35,4.1076909393
1546300740000,2018-12-31 23:59:00,BTCUSD,3686.38,3692.35,3685.7,3692.35,4.1076909393
1546300740000,2018-12-31 23:59:00,BTCUSD,3686.38,3692.35,3685.7,3692.35,4.1076909393
"#;
        let mut reader = csv::Reader::from_reader(data.as_bytes());
        let records: Vec<Record> = reader
            .deserialize::<CSVRecord>()
            .map(|result| Record::from(result.unwrap()))
            .collect();
        assert_eq!(records.len(), 3);
    }

    #[test]
    fn test() {
        thread::spawn(move || {
            execute("data.csv", 8080);
        });

        let result = get_btc_record("127.0.0.1:8080");
        assert!(result.is_ok(), "get_btc_record shouldn't return an error");
    }
}