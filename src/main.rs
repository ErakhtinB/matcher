extern crate csv;
extern crate matcher;

use csv::Reader;
use serde::Deserialize;
use std::env;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct OrderBuilder {
    order_type: matcher::order::OrderType,
    side: matcher::order::Side,
    price: u64,
    initial_qty: u64,
    user_id: u64,
}

fn routine(path: &str) -> Result<(), Box<dyn Error>> {
    let mut m = matcher::Matcher::new();
    let mut rdr = Reader::from_path(path)?;
    for result in rdr.deserialize() {
        let record: OrderBuilder = result?;
        m.proceed_record(matcher::order::Order::new(
            record.order_type,
            record.side,
            record.price,
            record.initial_qty,
            record.user_id,
        ));
    }
    Ok(())
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let l = args.len();
    if l != 2 {
        panic!("Args number is {} instead of 1", l - 1);
    }
    if let Err(e) = routine(&args[1]) {
        eprintln!("{}", e);
    }
}
