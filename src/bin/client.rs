use circular_queue::CircularQueue;
use clap::{App, Arg};
use ift611_project::client::*;
use ift611_project::logger::Logger;

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
                .takes_value(true)
                .value_name("STRATEGY")
                .possible_values(&["dummy"]),
        )
        .get_matches();

    let url = matches.value_of("URL").unwrap();
    let strategy: TradingStrategy = matches.value_of("strategy").unwrap().into();

    let logger = Logger::<DecisionLogs>::start("decicions.csv", 100);
    let mut queue = CircularQueue::with_capacity(100);
    queue.push(TradingPair::BTCUSD.get_record(url).unwrap());

    for record in TradingPair::BTCUSD.subscribe(url).unwrap() {
        queue.push(record);
        let decision = strategy.make_decision(&queue);
        logger.info(decision);
    }
}
