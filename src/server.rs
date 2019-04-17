use crate::logger::{Context, Logger};
use crate::Record;

use std::io::Write;
use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use tiny_http::{Method, Request, Response, Server};

#[derive(Copy, Clone)]
enum LogVariants {
    LoadingRecords,
    StartingPushServer,
    StartingSubscriptionServer,
    StartingHttpServer,
    MaxConnectionsInsufficient,
    RemovingConnection(SocketAddr),
}

impl Context for LogVariants {
    fn context_string(&self) -> String {
        match self {
            LogVariants::LoadingRecords => "Loading records data...".to_string(),
            LogVariants::StartingPushServer => "Starting push server...".to_string(),
            LogVariants::StartingSubscriptionServer => {
                "Starting subscription server...".to_string()
            }
            LogVariants::StartingHttpServer => "Starting http server...".to_string(),
            LogVariants::MaxConnectionsInsufficient => {
                "maximum number of connections is insufficient".to_string()
            }
            LogVariants::RemovingConnection(addr) => {
                format!("removing: {} from active connections", addr)
            }
        }
    }
}

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
const MAX_CONNECTIONS: usize = 10;

pub fn start(file: &str, http_port: u16) -> JoinHandle<()> {
    let logger = Logger::<LogVariants>::start("server_log.txt");

    let _result = logger.warning(LogVariants::LoadingRecords);
    let records = Arc::new(load_data(file));

    let streams = Arc::new(Mutex::new(Vec::with_capacity(MAX_CONNECTIONS)));

    let _result = logger.warning(LogVariants::StartingSubscriptionServer);
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
    let push_port = listener.local_addr().unwrap().port();
    {
        let logger = logger.clone();
        let streams = streams.clone();
        thread::spawn(move || {
            for stream in listener.incoming() {
                if let Ok(stream) = stream {
                    let mut v = streams.lock().unwrap();
                    if v.len() <= MAX_CONNECTIONS {
                        stream
                            .set_write_timeout(Some(PERIOD_DURATION / MAX_CONNECTIONS as u32))
                            .unwrap();
                        v.push(stream);
                    } else {
                        let _result = logger.warning(LogVariants::MaxConnectionsInsufficient);
                    }
                }
            }
        });
    }

    let _result = logger.warning(LogVariants::StartingHttpServer);
    let start_time = Instant::now();
    let server = Server::http((Ipv4Addr::LOCALHOST, http_port)).unwrap();
    let http_server_handle = {
        let logger = logger.clone();
        let records = records.clone();

        thread::spawn(move || {
            for request in server.incoming_requests() {
                handle_request(logger.clone(), request, start_time, &records, push_port);
            }
        })
    };

    let _result = logger.warning(LogVariants::StartingPushServer);
    {
        let logger = logger.clone();
        let streams = streams.clone();
        let start_time = start_time;
        let records = records.clone();
        thread::spawn(move || {
            periodic_push(logger, &streams, start_time, &records);
        });
    }

    http_server_handle
}

fn periodic_push(
    mut logger: Logger<LogVariants>,
    streams: &Mutex<Vec<TcpStream>>,
    start_time: Instant,
    data: &[Record],
) {
    let mut time_to_wake = Instant::now();
    loop {
        push_data(&mut logger, streams, start_time, data);
        time_to_wake += PERIOD_DURATION;
        thread::sleep(time_to_wake - Instant::now());
    }
}

fn push_data(
    logger: &mut Logger<LogVariants>,
    streams: &Mutex<Vec<TcpStream>>,
    start_time: Instant,
    data: &[Record],
) {
    let mut streams = streams.lock().unwrap();
    static mut TO_REMOVE: [usize; MAX_CONNECTIONS] = [0; MAX_CONNECTIONS];
    static mut CURR: usize = 0;
    let current_data = get_current_data(start_time, data).into_bytes();
    for (i, stream) in streams.iter_mut().enumerate() {
        let result = stream.write_all(&current_data);
        if result.is_err() {
            unsafe {
                TO_REMOVE[CURR] = i;
                CURR += 1;
            }
        }
    }
    unsafe {
        for i in &TO_REMOVE[0..CURR] {
            let addr = streams[*i].peer_addr().unwrap();
            let _result = logger.info(LogVariants::RemovingConnection(addr));
            streams.remove(*i);
        }
        CURR = 0;
    }
}

fn load_data(filename: &str) -> Vec<Record> {
    let mut reader = csv::Reader::from_path(filename).unwrap();

    reader
        .deserialize::<CSVRecord>()
        .take(100)
        .map(|result| Record::from(result.unwrap()))
        .collect()
}

fn handle_request(
    _logger: Logger<LogVariants>,
    req: Request,
    start_time: Instant,
    btc_records: &[Record],
    push_port: u16,
) {
    let response = match (req.method(), req.url()) {
        (&Method::Get, "/") => Response::from_string("Hello!"),
        (&Method::Get, "/BTCUSD") => {
            Response::from_string(get_current_data(start_time, btc_records))
        }
        (&Method::Post, "/subscribe/BTCUSD") => Response::from_string(push_port.to_string()),
        (method, url) => Response::from_string(format!("Invalid request: {} at {}", method, url)).with_status_code(404),
    };
    req.respond(response).unwrap();
}

fn get_current_data(start_time: Instant, records: &[Record]) -> String {
    let i = (start_time.elapsed().as_micros() / u128::from(PERIOD)) as usize;
    serde_json::to_string(&records[i]).unwrap()
}

mod test {
    #[allow(unused_imports)]
    use super::{start, CSVRecord, Deserialize, Record};
    #[allow(unused_imports)]
    use crate::get_btc_record;
    #[allow(unused_imports)]
    use crate::subscribe_btc;
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
        start("data.csv", 8080);
        query_test();
        subscribe_test();
    }

    #[allow(dead_code)]
    fn query_test() {
        let result = get_btc_record("http://127.0.0.1:8080");
        assert!(
            result.is_ok(),
            format!("get_btc_record shouldn't return an error: {:?}", result)
        );
    }

    #[allow(dead_code)]
    fn subscribe_test() {
        let result = subscribe_btc("http://127.0.0.1:8080");
        assert!(
            result.is_ok(),
            format!("subscribe_btc shouldn't return an error: {:?}", result)
        );
    }
}
