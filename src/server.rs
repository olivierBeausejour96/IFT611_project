extern crate csv;
extern crate serde;
extern crate serde_json;
extern crate tiny_http;
extern crate bincode;

use bincode::{serialize, deserialize};
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tiny_http::{Method, Request, Response, Server};

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

#[derive(Serialize, Deserialize)]
struct CSVRecord {
    _timestamp: String,
    _date: String,
    _symbol: String,
    open: f32,
    high: f32,
    low: f32,
    close: f32,
    volume: f64,
}

#[derive(Serialize, Deserialize)]
struct Record {
    open: f32,
    high: f32,
    low: f32,
    close: f32,
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

#[derive(Deserialize)]
struct Client {
    ip: String,
    port: u16,
}

fn load_data(filename: &str) -> Vec<Record> {
    let mut reader = csv::Reader::from_path(filename).unwrap();

    reader.deserialize::<CSVRecord>().map(|result| Record::from(result.unwrap())).collect()
}

fn handle_request(mut req: Request, start_time: Instant, btc_records: &[Record], clients: &Mutex<Vec<Client>>) {
    let response = match (req.method(), req.url()) {
        (&Method::Get, "/") => Response::from_string("Hello!"),
        (&Method::Get, "/BTCUSD") => Response::from_string(current_data(start_time, btc_records)),
        (&Method::Post, "/subscribe/BTCUSD") => match get_client(&mut req) {
            Ok(client) => {
                clients.lock().unwrap().push(client);
                Response::from_string("")
            }
            Err(e) => {
                eprintln!("{}", e);
                Response::from_string("").with_status_code(500)
            }
        },
        _ => Response::from_string("").with_status_code(404),
    };
    req.respond(response).unwrap();
}

fn get_client(req: &mut Request) -> Result<Client> {
    let mut content = String::new();
    req.as_reader()
        .read_to_string(&mut content)
        .or_else(|e| Err(e.into()))
        .and_then(|_| serde_json::from_str(&content).map_err(|e| e.into()))
}

fn current_data(start_time: Instant, records: &[Record]) -> String {
    let i = (start_time.elapsed().as_millis() / 2) as usize;
    let record = &records[i];
    format!(
        "{},{},{},{},{}",
        record.open, record.high, record.low, record.close, record.volume
    )
}


pub fn execute() {
    let file = "data.csv";
    let records = Arc::new(load_data(file));
    let clients = Arc::new(Mutex::new(vec![]));

    let port = 7878;
    let addr = format!("127.0.0.1:{}", port);

    let server = Server::http(addr).unwrap();
    let start_time = Instant::now();

    for request in server.incoming_requests() {
        handle_request(request, start_time, records.as_ref(), clients.as_ref());
    }
}