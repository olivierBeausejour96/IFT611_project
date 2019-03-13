extern crate bincode;
extern crate csv;
extern crate failure;
extern crate serde;
extern crate serde_json;
extern crate tiny_http;

// use bincode::{serialize, deserialize};
use crate::common::Record;
use failure::Error as AnyError;
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use std::sync::Mutex;
use std::time::Instant;
use tiny_http::{Method, Request, Response, Server};

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
    _ip: String,
    _port: u16,
}

pub fn execute(file: &str, port: u16) {
    let records = load_data(file);
    let clients = Mutex::new(vec![]);

    let server = Server::http((Ipv4Addr::LOCALHOST, port)).unwrap();
    let start_time = Instant::now();

    for request in server.incoming_requests() {
        handle_request(request, start_time, &records, &clients);
    }
}

fn load_data(filename: &str) -> Vec<Record> {
    let mut reader = csv::Reader::from_path(filename).unwrap();

    reader
        .deserialize::<CSVRecord>()
        .map(|result| Record::from(result.unwrap()))
        .collect()
}

fn handle_request(
    mut req: Request,
    start_time: Instant,
    btc_records: &[Record],
    clients: &Mutex<Vec<Client>>,
) {
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

fn get_client(req: &mut Request) -> Result<Client, AnyError> {
    let mut content = String::new();
    req.as_reader().read_to_string(&mut content)?;
    Ok(serde_json::from_str(&content)?)
}

fn current_data(start_time: Instant, records: &[Record]) -> String {
    let i = (start_time.elapsed().as_millis() / 2) as usize;
    records[i].to_csv_str()
}
