// Author: Karim Elmougi

use circular_queue::CircularQueue;
use clap::{App, Arg};
use ift611_project::client::*;
use ift611_project::logger::Logger;
use std::io::stdout;
use std::fs::File;

fn main() {
    let matches = App::new("HFT Client")
        .version("0.1.0")
        .author("Karim Elmougi <karim@elmougi.dev>")
        .arg(
            Arg::with_name("URL")
                .help("Sets the url of the server")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("strategy")
                .help("Sets the trading strategy to use")
                .long("strategy")
                .takes_value(true)
                .value_name("STRATEGY")
                .possible_values(&["dummy"]),
        )
        .arg(
            Arg::with_name("writer")
                .help("Sets where decisions are written")
                .long("writer")
                .takes_value(true)
                .value_name("WRITER")
                .possible_values(&["file", "stdout"]),
        )
        .get_matches();

    let url = matches.value_of("URL").unwrap();
    let strategy: TradingStrategy = matches.value_of("strategy").unwrap_or("dummy").into();

    let logger = match matches.value_of("writer").unwrap_or("stdout") {
        "file" => Logger::start(File::create("decisions_log.csv").unwrap(), 100),
        "stdout" => Logger::start(stdout(), 100),
        _ => Logger::start(stdout(), 100),
    };

    let mut queue = CircularQueue::with_capacity(100);

    for record in TradingPair::BTCUSD.subscribe(url).unwrap() {
        queue.push(record);
        let decision = strategy.make_decision(&queue);
        logger.info(decision);
    }
}
