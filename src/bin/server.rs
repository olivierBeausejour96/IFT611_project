// Author: Karim Elmougi

use ift611_project::server::ServerBuilder;

use clap::{App, Arg};

fn main() {
    let matches = App::new("HFT Server")
        .version("0.1.0")
        .author("Karim Elmougi <karim@elmougi.dev>")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input data file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("period")
                .help("Sets the period in μs with which to push data to subscribers")
                .long("period")
                .takes_value(true)
                .value_name("PERIOD"),
        )
        .arg(
            Arg::with_name("max-subscribers")
                .help("Sets the maximum number of subscribers at one time")
                .long("max-subscribers")
                .takes_value(true)
                .value_name("VALUE"),
        )
        .arg(
            Arg::with_name("log-queue-size")
                .help("Sets the size of the logger's message queue")
                .long("log-queue-size")
                .takes_value(true)
                .value_name("SIZE"),
        )
        .arg(
            Arg::with_name("port")
                .help("Sets the port to use")
                .short("p")
                .long("port")
                .takes_value(true)
                .value_name("PORT"),
        )
        .get_matches();

    let mut builder = ServerBuilder::new(matches.value_of("INPUT").unwrap());

    if let Some(port) = matches.value_of("port") {
        builder = builder.with_http_port(
            port.parse()
                .unwrap_or_else(|_| panic!("invalid port value: {}", port)),
        );
    }

    if let Some(period) = matches.value_of("period") {
        builder = builder.with_period(
            period
                .parse()
                .unwrap_or_else(|_| panic!("invalid period value: {}", period)),
        );
    }

    if let Some(max_sub) = matches.value_of("max-subscribers") {
        builder = builder.with_max_subscriber_count(
            max_sub
                .parse()
                .unwrap_or_else(|_| panic!("invalid max-subscribers value: {}", max_sub)),
        );
    }

    if let Some(log_queue_size) = matches.value_of("log-queue-size") {
        builder = builder.with_logger_queue_size(
            log_queue_size
                .parse()
                .unwrap_or_else(|_| panic!("invalid log-queue-size value: {}", log_queue_size)),
        );
    }

    builder.build_and_start().join().unwrap();
}
