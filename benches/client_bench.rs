// Author: Karim Elmougi

#[macro_use]
extern crate criterion;

use circular_queue::CircularQueue;
use criterion::{black_box, Criterion};
use ift611_project::client::*;

fn record_deserialization_benchmark(c: &mut Criterion) {
    c.bench_function("client-record_deserialization", move |b| {
        b.iter(|| Record::from_csv_str(black_box("1546299180000,2018-12-31 23:33:00,BTCUSD,3681.32,3681.32,3680.63,3681.13,83.97565747")))
    });
}

fn buffer_push_benchmark(c: &mut Criterion) {
    let mut queue = CircularQueue::with_capacity(100);
    let record = Record::from_csv_str(
        "1546299180000,2018-12-31 23:33:00,BTCUSD,3681.32,3681.32,3680.63,3681.13,83.97565747",
    )
    .unwrap();

    c.bench_function("client-ring_buffer", move |b| {
        b.iter(|| queue.push(black_box(record)))
    });
}

fn dummy_strategy_benchmark(c: &mut Criterion) {
    let mut queue = CircularQueue::with_capacity(100);
    let record = Record::from_csv_str(
        "1546299180000,2018-12-31 23:33:00,BTCUSD,3681.32,3681.32,3680.63,3681.13,83.97565747",
    )
    .unwrap();

    for _ in 0..100 {
        queue.push(record);
    }

    c.bench_function("client-dummy_strategy", move |b| {
        b.iter(|| TradingStrategy::Dummy.make_decision(black_box(&queue)))
    });
}

criterion_group!(
    benches,
    record_deserialization_benchmark,
    buffer_push_benchmark,
    dummy_strategy_benchmark
);
criterion_main!(benches);
