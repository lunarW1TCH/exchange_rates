mod api;
mod args;
mod currencies;

use args::Args;
use clap::Parser;

use crate::currencies::read_currencies;
use std::error::Error;

// TODO: enum for error types

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    // TODO - https://rust-cli.github.io/book/tutorial/cli-args.html

    let _client = reqwest::Client::new();
    let _data = read_currencies()?;

    // println!("{:#?}", data);
    // println!("{}", is_code_supported(&data, "ZWL".to_string()));
    // println!("{}", is_code_supported(&data, "ZWLs".to_string()));

    dbg!(args);

    Ok(())
}
