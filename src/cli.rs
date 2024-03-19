use crate::currencies::Currencies;
use clap::Parser;
use std::error::Error;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
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

/// Function to be called as a return result when user requests a list of currencies with their corresponding codes.
pub fn list_handler(currencies: &Currencies) -> Result<(), Box<dyn Error>> {
    for c in currencies.iter() {
        println!("{} - {}", c.code, c.name);
    }
    Ok(())
}
