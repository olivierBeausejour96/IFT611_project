use ift611_project::server::ServerBuilder;

fn main() {
    ServerBuilder::new("data.csv")
        .with_http_port(8080)
        .build_and_start()
        .join()
        .unwrap();
}
