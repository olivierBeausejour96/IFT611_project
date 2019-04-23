extern crate ift611_project;

use ift611_project::client::*;
use ift611_project::server::*;

#[test]
fn test() {
    ServerBuilder::new("data.csv")
        .with_http_port(8080)
        .with_max_records_amount(Some(100))
        .with_period(100_000)
        .build_and_start();
    query_test();
    subscribe_test();
}

fn query_test() {
    let result = TradingPair::BTCUSD.get_record("http://127.0.0.1:8080");
    assert!(
        result.is_ok(),
        format!(
            "BTCUSD.get_record() shouldn't return an error: {:?}",
            result
        )
    );
}

fn subscribe_test() {
    let result = TradingPair::BTCUSD.subscribe("http://127.0.0.1:8080");
    assert!(
        result.is_ok(),
        format!("BTCUSD.subscribe() shouldn't return an error: {:?}", result)
    );

    let mut connection = result.unwrap();
    let result = connection.next();
    assert!(
        result.is_some(),
        format!(
            "getting the next record shouldn't return an error: {:?}",
            result
        )
    );
}
