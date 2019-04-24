use crate::logger::{Context, Logger};
use crossbeam::channel::tick;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use tiny_http::{Method, Request, Response, Server};

#[derive(Copy, Clone)]
pub enum ServerLogs {
    LoadingRecords,
    StartingPushServer,
    StartingSubscriptionServer,
    StartingHttpServer,
    MaxSubscribersInsufficient,
    MissedPushDeadline,
    ClientUnreachable(SocketAddr),
    PushingToSubscriber(SocketAddr),
    AddingSubscriber(SocketAddr),
}

impl Context for ServerLogs {
    fn context_string(&self) -> String {
        match self {
            ServerLogs::LoadingRecords => "Loading records data...".to_string(),
            ServerLogs::StartingPushServer => "Starting push server...".to_string(),
            ServerLogs::StartingSubscriptionServer => "Starting subscription server...".to_string(),
            ServerLogs::StartingHttpServer => "Starting http server...".to_string(),
            ServerLogs::MaxSubscribersInsufficient => {
                "maximum number of connections is insufficient".to_string()
            }
            ServerLogs::MissedPushDeadline => "missed a deadline on subscription push".to_string(),
            ServerLogs::ClientUnreachable(addr) => format!("could not reach client: {}", addr),
            ServerLogs::PushingToSubscriber(addr) => {
                format!("pushing data to subscriber: {}", addr)
            }
            ServerLogs::AddingSubscriber(addr) => format!("adding subscriber: {}", addr),
        }
    }
}

pub struct ServerBuilder {
    data_file: String,
    http_port: u16,
    period: u64,
    max_subscriber_count: usize,
    logger_queue_size: usize,
    max_records_amount: Option<usize>,
}

impl ServerBuilder {
    pub fn new(data_file: &str) -> Self {
        ServerBuilder {
            data_file: data_file.to_owned(),
            http_port: 80,
            period: 1000,
            max_subscriber_count: 10,
            logger_queue_size: 100,
            max_records_amount: None,
        }
    }

    pub fn with_http_port(self, http_port: u16) -> Self {
        Self { http_port, ..self }
    }

    pub fn with_period(self, period: u64) -> Self {
        Self { period, ..self }
    }

    pub fn with_max_subscriber_count(self, max_subscriber_count: usize) -> Self {
        Self {
            max_subscriber_count,
            ..self
        }
    }

    pub fn with_logger_queue_size(self, logger_queue_size: usize) -> Self {
        Self {
            logger_queue_size,
            ..self
        }
    }

    pub fn with_max_records_amount(self, max_records_amount: Option<usize>) -> Self {
        Self {
            max_records_amount,
            ..self
        }
    }

    pub fn build_and_start(self) -> JoinHandle<()> {
        let logger = Logger::<ServerLogs>::start("server_log.csv", self.logger_queue_size);

        logger.info(ServerLogs::LoadingRecords);
        let records = Arc::new(load_data(&self.data_file, self.max_records_amount));
        let streams = Arc::new(Mutex::new(Vec::with_capacity(self.max_subscriber_count)));

        let (_, push_port) = start_subscription_server(&self, logger.clone(), streams.clone());

        let start_time = Instant::now();
        let http_server_handle = start_http_server(
            logger.clone(),
            self.http_port,
            push_port,
            start_time,
            self.period,
            records.clone(),
        );

        start_push_server(
            logger.clone(),
            start_time,
            self.period,
            records.clone(),
            streams.clone(),
        );

        http_server_handle
    }
}

fn start_http_server(
    logger: Logger<ServerLogs>,
    http_port: u16,
    push_port: u16,
    start_time: Instant,
    period: u64,
    records: Arc<Vec<String>>,
) -> JoinHandle<()> {
    logger.info(ServerLogs::StartingHttpServer);

    let server = Server::http((Ipv4Addr::LOCALHOST, http_port)).unwrap();

    thread::spawn(move || {
        for request in server.incoming_requests() {
            handle_request(
                logger.clone(),
                request,
                start_time,
                &records,
                push_port,
                period,
            );
        }
    })
}

fn start_subscription_server(
    builder: &ServerBuilder,
    logger: Logger<ServerLogs>,
    streams: Arc<Mutex<Vec<TcpStream>>>,
) -> (JoinHandle<()>, u16) {
    logger.info(ServerLogs::StartingSubscriptionServer);

    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
    let push_port = listener.local_addr().unwrap().port();
    let max_subscriber_count = builder.max_subscriber_count;
    let write_timeout = Duration::from_micros(builder.period / builder.max_subscriber_count as u64);

    let handle = thread::spawn(move || {
        for stream in listener.incoming() {
            if let Ok(stream) = stream {
                let mut v = streams.lock().unwrap();
                if v.len() <= max_subscriber_count {
                    logger.info(ServerLogs::AddingSubscriber(stream.peer_addr().unwrap()));
                    stream.set_write_timeout(Some(write_timeout)).unwrap();
                    stream.set_nonblocking(true).unwrap();
                    v.push(stream);
                } else {
                    logger.warning(ServerLogs::MaxSubscribersInsufficient);
                }
            }
        }
    });

    (handle, push_port)
}

fn start_push_server(
    logger: Logger<ServerLogs>,
    start_time: Instant,
    period: u64,
    records: Arc<Vec<String>>,
    streams: Arc<Mutex<Vec<TcpStream>>>,
) -> JoinHandle<()> {
    logger.info(ServerLogs::StartingPushServer);

    thread::spawn(move || {
        let period_duration = Duration::from_micros(period);
        let ticker = tick(period_duration);
        while let Ok(_wake_time) = ticker.recv() {
            push_data(&logger, &streams, start_time, period, &records);
        }
    })
}

pub fn push_data(
    _logger: &Logger<ServerLogs>,
    streams: &Mutex<Vec<TcpStream>>,
    start_time: Instant,
    period: u64,
    data: &[String],
) {
    let mut streams = streams.lock().unwrap();
    let current_data = get_current_data(start_time, period, data).as_bytes();
    streams.retain(|mut stream| {
        let result = stream.write_all(&current_data);
        result.is_ok()
    });
}

fn load_data(filename: &str, max_records_amount: Option<usize>) -> Vec<String> {
    let file = File::open(filename).unwrap_or_else(|_| panic!("invalid filename: {}", filename));
    let reader = BufReader::new(&file);

    match max_records_amount {
        Some(amount) => reader
            .lines()
            .skip(1)
            .map(|result| result.unwrap() + "\n")
            .take(amount)
            .collect(),
        None => reader
            .lines()
            .skip(1)
            .map(|result| result.unwrap() + "\n")
            .collect(),
    }
}

fn handle_request(
    logger: Logger<ServerLogs>,
    req: Request,
    start_time: Instant,
    btc_records: &[String],
    push_port: u16,
    period: u64,
) {
    let response = match (req.method(), req.url()) {
        (&Method::Get, "/") => Response::from_string("Hello!"),
        (&Method::Get, "/BTCUSD") => {
            Response::from_string(get_current_data(start_time, period, btc_records))
        }
        (&Method::Post, "/subscribe/BTCUSD") => Response::from_string(push_port.to_string()),
        (method, url) => Response::from_string(format!("Invalid request: {} at {}", method, url))
            .with_status_code(404),
    };

    let addr = *req.remote_addr();
    if req.respond(response).is_err() {
        logger.warning(ServerLogs::ClientUnreachable(addr));
    }
}

fn get_current_data(start_time: Instant, period: u64, data: &[String]) -> &str {
    let i = (start_time.elapsed().as_micros() / u128::from(period)) as usize;
    &data[i]
}
