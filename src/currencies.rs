use serde::{Deserialize, Serialize};

const CURRENCIES_PATH: &str = "./api_data/physical_currency_list.csv";

#[derive(Serialize, Deserialize, Debug)]
pub struct Currency {
    pub code: String,
    pub name: String,
}

pub type Currencies = Vec<Currency>;

pub fn read_currencies() -> Result<Currencies, csv::Error> {
    let mut reader = csv::Reader::from_path(CURRENCIES_PATH)?;
    let mut currencies: Currencies = Vec::new();

    for result in reader.deserialize() {
        let record: Currency = result?;
        currencies.push(record);
    }

    Ok(currencies)
}

pub fn is_code_supported(currencies: &Currencies, code: &String) -> bool {
    let currency = currencies.iter().find(|el| el.code.eq(code));

    currency.is_some()
}
