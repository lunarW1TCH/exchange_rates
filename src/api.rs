use std::error::Error;

use reqwest::Client;

use crate::currencies::{is_code_supported, Currencies};

const API_URL: &str = "https://www.alphavantage.co/query?function=CURRENCY_EXCHANGE_RATE";

pub async fn fetch_exchange_rate(
    client: &Client,
    from: String,
    to: String,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn construct_api_url(from: &str, to: &str) -> String {
    format!("{API_URL}&from_currency={from}&to_currency={to}&apikey=test")
}
