mod api;
mod cli;
mod currencies;

use clap::Parser;
use cli::Args;

use crate::{cli::list_handler, currencies::read_currencies};
use std::error::Error;

// TODO: enum for error types

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli_args = Args::parse();
    let data = read_currencies()?;

    if cli_args.list {
        return list_handler(&data);
    }

    let _client = reqwest::Client::new();

    // println!("{:#?}", data);
    // println!("{}", is_code_supported(&data, "ZWL".to_string()));
    // println!("{}", is_code_supported(&data, "ZWLs".to_string()));

    dbg!(cli_args);

    Ok(())
}
