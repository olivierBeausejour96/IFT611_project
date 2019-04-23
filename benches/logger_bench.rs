#[macro_use]
extern crate criterion;

use criterion::{black_box, Criterion, ParameterizedBenchmark};
use ift611_project::logger::{Context, Logger};
use std::thread;
use std::time::Duration;

#[derive(Copy, Clone)]
enum TestLogs {
    SmallLog,
    MediumLog,
    BigLog,
    HugeLog,
}

impl Context for TestLogs {
    fn context_string(&self) -> String {
        match self {
            TestLogs::SmallLog => (0..2).map(|_| "I am a string").collect(),
            TestLogs::MediumLog => (0..10).map(|_| "I am a string").collect(),
            TestLogs::BigLog => (0..20).map(|_| "I am a string").collect(),
            TestLogs::HugeLog => {
                thread::sleep(Duration::from_millis(5));
                "I am a string".to_string()
            }
        }
    }
}

const QUEUE_SIZE: usize = 100;

fn small_log(c: &mut Criterion) {
    c.bench_function("logger-small_logs", move |b| {
        let logger = Logger::<TestLogs>::start("test_log.csv", QUEUE_SIZE);
        b.iter(|| logger.info(black_box(TestLogs::SmallLog)))
    });
}

fn medium_log(c: &mut Criterion) {
    c.bench_function("logger-medium_logs", move |b| {
        let logger = Logger::<TestLogs>::start("test_log.csv", QUEUE_SIZE);
        b.iter(|| logger.info(black_box(TestLogs::MediumLog)))
    });
}

fn big_log(c: &mut Criterion) {
    c.bench_function("logger-big_logs", move |b| {
        let logger = Logger::<TestLogs>::start("test_log.csv", QUEUE_SIZE);
        b.iter(|| logger.info(black_box(TestLogs::BigLog)))
    });
}

fn compare_queue_size(c: &mut Criterion) {
    c.bench(
        "logger-compare_queue_size",
        ParameterizedBenchmark::new(
            "small_queue",
            move |b, _| {
                let logger_small = Logger::<TestLogs>::start("test_log.csv", 1);
                b.iter(|| logger_small.info(black_box(TestLogs::HugeLog)))
            },
            0..1,
        )
        .with_function("big_queue", move |b, _| {
            let logger_big = Logger::<TestLogs>::start("test_log.csv", 50);
            b.iter(|| logger_big.info(black_box(TestLogs::HugeLog)))
        }),
    );
}

criterion_group!(benches, small_log, medium_log, big_log, compare_queue_size);
criterion_main!(benches);
