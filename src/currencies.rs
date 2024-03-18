use serde::{Deserialize, Serialize};
use std::error::Error;

const CURRENCIES_PATH: &str = "./api_data/physical_currency_list.csv";

#[derive(Serialize, Deserialize, Debug)]
pub struct Currency {
    code: String,
    name: String,
}

pub async fn read_currencies() -> Result<Vec<Currency>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(CURRENCIES_PATH)?;
    let mut currencies: Vec<Currency> = Vec::new();

    for result in reader.deserialize() {
        let record: Currency = result?;
        currencies.push(record);
    }

    Ok(currencies)
}
