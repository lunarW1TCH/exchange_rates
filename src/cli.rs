use std::error::Error;

use clap::Parser;

use crate::currencies::Currencies;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Prints a list of available country codes with their corresponding name.
    #[arg(short, long, exclusive = true)]
    pub list: bool,

    /// The destination currency for the exchange rate. For example: `--to=USD`
    #[arg(short, long, required = false, requires = "from", requires = "amount")]
    pub to: Option<String>,

    /// The currency you would like to get the exchange rate for. For example: `--from=PLN`
    #[arg(short, long, required = false, requires = "to", requires = "amount")]
    pub from: Option<String>,

    /// Amount you would like to convert. For example: `--amount=21.37`
    #[arg(short, long, required = false, requires = "from", requires = "to")]
    pub amount: Option<f32>,
}

pub fn list_handler(currencies: &Currencies) -> Result<(), Box<dyn Error>> {
    for c in currencies.iter() {
        println!("{} - {}", c.code, c.name);
    }
    Ok(())
}
