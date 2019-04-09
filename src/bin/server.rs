use ift611_project::server::start;

fn main() {
    start("data.csv", 8080).join().unwrap();
}
