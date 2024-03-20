mod api;
mod cli;
mod currencies;
mod errors;

use crate::{
    api::fetch_exchange_rate,
    cli::list_handler,
    cli::Args,
    currencies::{is_code_supported, read_currencies},
    errors::{FROM_CODE_ERROR, TO_CODE_ERROR},
};
use clap::Parser;
use dotenv::dotenv;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let cli_args = Args::parse();
    let data = read_currencies()?;

    if cli_args.list {
        return list_handler(&data);
    }

    let from = cli_args.from.unwrap_or("".to_string().to_uppercase());
    let to = cli_args.to.unwrap_or("".to_string().to_uppercase());
    let amount = cli_args.amount.unwrap_or(1.0);

    if !is_code_supported(&data, &from) {
        return Err(Box::new(FROM_CODE_ERROR));
    } else if !is_code_supported(&data, &to) {
        return Err(Box::new(TO_CODE_ERROR));
    }

    let client = reqwest::Client::new();

    let result = fetch_exchange_rate(&client, from, to, amount).await?;

    println!("\n{}", result);

    Ok(())
}
