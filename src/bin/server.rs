use ift611_project::common::get_btc_record;
use ift611_project::server::start;

fn main() {
    start("data.csv", 8080);
    println!("Server started");
    let result = get_btc_record("http://127.0.0.1:8080");
    assert!(
        result.is_ok(),
        format!("get_btc_record shouldn't return an error: {:?}", result)
    );
}
