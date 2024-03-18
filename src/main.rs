use std::error::Error;

mod currencies;
mod http;

// TODO: enum for error types

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let data = currencies::read_currencies().await?;
    println!("{:#?}", data);

    Ok(())
}
