fn main() {
    let record = ift611_project::common::get_btc_record("127.0.0.1:8080").unwrap();
    println!("{:?}", record);
}
