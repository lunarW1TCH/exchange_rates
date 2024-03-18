use std::error::Error;

use crate::currencies::{is_code_supported, read_currencies};

mod currencies;
mod http;

// TODO: enum for error types

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let data = read_currencies()?;

    println!("{:#?}", data);
    println!("{}", is_code_supported(&data, "ZWL".to_string()));
    println!("{}", is_code_supported(&data, "ZWLs".to_string()));

    Ok(())
}
