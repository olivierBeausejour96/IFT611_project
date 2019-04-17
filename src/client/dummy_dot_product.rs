use crate::Record;

#[derive(Debug, PartialEq)]
pub enum Action {
    Sell,
    Buy,
    Hold,
}

/// # Brief
///
/// Given a window period of 100 time units, return either a buy or sell action using super dooper complex algorithm
///
/// # Example
///
/// ```
/// use ift611_project::client::dummy_dot_product::{Action, get_decision};
/// use ift611_project::Record;
/// let mut expected_sell_data = [Record {open: 32.0, high: 32.0, low: 32.0, close: 32.0, volume: 64.0}; 100];
/// (0..100).into_iter()
/// .for_each(
///     |x|
///     expected_sell_data[x] = Record {open: (x as f32), high: ((x+1) as f32), low: (x as f32), close: ((x+1) as f32), volume: 64.0 });
/// let a = get_decision(&expected_sell_data);
/// assert_eq!(a, Action::Sell);
/// ```
pub fn get_decision(data: &[Record; 100]) -> Action {
    let first = &data.first().unwrap();
    let last = &data.last().unwrap();

    // super smart ai decision process
    if last.close < first.open {
        Action::Buy
    } else {
        Action::Sell
    }
}
