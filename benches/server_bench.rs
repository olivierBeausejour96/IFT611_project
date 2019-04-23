#[macro_use]
extern crate criterion;

use ift611_project::logger::Logger;
use ift611_project::server::{push_data, ServerLogs};

use criterion::Criterion;

use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

fn start_dummy_server(streams: Arc<Mutex<Vec<TcpStream>>>) -> u16 {
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();

    let push_port = listener.local_addr().unwrap().port();

    thread::spawn(move || {
        for stream in listener.incoming() {
            if let Ok(stream) = stream {
                streams.lock().unwrap().push(stream);
            }
        }
    });

    push_port
}

fn populate_streams(port: u16, streams: Arc<Mutex<Vec<TcpStream>>>) {
    let mut streams = streams.lock().unwrap();
    for _ in 0..100 {
        let stream = TcpStream::connect((Ipv4Addr::LOCALHOST, port)).unwrap();
        stream.set_nonblocking(true).unwrap();
        streams.push(stream);
    }
}

fn push_data_benchmark(c: &mut Criterion) {
    let streams = Arc::new(Mutex::new(Vec::with_capacity(10)));
    let port = start_dummy_server(streams.clone());
    populate_streams(port, streams.clone());

    let logger = Logger::<ServerLogs>::start("test_log.csv", 100);
    let data: Vec<String> = (0..1000).map(|_| "I am a string!".to_string()).collect();
    let start_time = Instant::now();

    c.bench_function("server-push_data", move |b| {
        b.iter(|| push_data(&logger, &streams, start_time, 100_000, &data))
    });
}

criterion_group!(benches, push_data_benchmark);
criterion_main!(benches);
