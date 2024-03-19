mod api;
mod cli;
mod currencies;
mod errors;

use clap::Parser;
use errors::{FROM_CODE_ERROR, TO_CODE_ERROR};
use std::error::Error;

use crate::{
    cli::list_handler,
    currencies::{is_code_supported, read_currencies},
};
use cli::Args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli_args = Args::parse();
    let data = read_currencies()?;

    if cli_args.list {
        return list_handler(&data);
    }

    let from = cli_args.from.unwrap_or("".to_string());
    let to = cli_args.to.unwrap_or("".to_string());
    let amount = cli_args.amount.unwrap_or(1.0);

    if !is_code_supported(&data, &from) {
        return Err(Box::new(FROM_CODE_ERROR));
    } else if !is_code_supported(&data, &to) {
        return Err(Box::new(TO_CODE_ERROR));
    }

    let _client = reqwest::Client::new();

    Ok(())
}
