use circular_queue::CircularQueue;
use clap::{App, Arg};
use ift611_project::client::*;
use ift611_project::logger::{Context, Logger};

#[derive(Debug)]
enum DecisionLogs {
    Buy,
    Sell,
    Wait,
}

impl Context for DecisionLogs {
    fn context_string(&self) -> String {
        format!("{:?}", self)
    }
}

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
        .get_matches();

    let url = matches.value_of("URL").unwrap();

    let logger = Logger::<DecisionLogs>::start("decicions.csv", 100);
    let mut buffer = CircularQueue::with_capacity(100);
    buffer.push(TradingPair::BTCUSD.get_record(url).unwrap());
    let treshold = 5.0;

    for record in TradingPair::BTCUSD.subscribe(url).unwrap() {
        buffer.push(record);
        let diff = buffer.iter().nth(0).unwrap().close - buffer.iter().last().unwrap().close;

        let decision = if diff > treshold {
            DecisionLogs::Buy
        } else if diff < treshold {
            DecisionLogs::Sell
        } else {
            DecisionLogs::Wait
        };

        logger.info(decision);
    }
}
