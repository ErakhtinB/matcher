extern crate csv;
extern crate matcher;

use clap::{Arg, ArgAction, Command};
use csv::Reader;
use serde::Deserialize;
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct OrderBuilder {
    order_type: matcher::order::OrderType,
    side: matcher::order::Side,
    price: u64,
    initial_qty: u64,
    user_id: u64,
}

fn process_csv(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let mut matcher = matcher::Matcher::new();
    let mut reader = Reader::from_path(path)?;

    for (index, result) in reader.deserialize::<OrderBuilder>().enumerate() {
        match result {
            Ok(record) => {
                matcher.proceed_record(matcher::order::Order::new(
                    record.order_type,
                    record.side,
                    record.price,
                    record.initial_qty,
                    record.user_id,
                ));
            }
            Err(e) => {
                eprintln!("Error at record {}: {}", index + 1, e);
                return Err(e.into());
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("Matcher")
        .version("0.1.0")
        .author("Author")
        .about("A trading order matching engine")
        .arg(
            Arg::new("input")
                .help("Input CSV file with order data")
                .required(true)
                .action(ArgAction::Set),
        )
        .get_matches();

    let input_path = PathBuf::from(matches.get_one::<String>("input").unwrap());
    if !input_path.exists() {
        eprintln!(
            "Error: Input file '{}' does not exist",
            input_path.display()
        );
        return Err("File not found".into());
    }

    match process_csv(&input_path) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Error processing file: {}", e);
            Err(e)
        }
    }
}
