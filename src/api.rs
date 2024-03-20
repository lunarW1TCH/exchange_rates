use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

const BASE_URL: &str = "https://www.alphavantage.co/query?function=CURRENCY_EXCHANGE_RATE";

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct ResponseData {
    #[serde(rename = "Realtime Currency Exchange Rate")]
    pub data: ResponseBody,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct ResponseBody {
    #[serde(rename = "1. From_Currency Code")]
    pub from_code: String,

    #[serde(rename = "2. From_Currency Name")]
    pub from_name: String,

    #[serde(rename = "3. To_Currency Code")]
    pub to_code: String,

    #[serde(rename = "4. To_Currency Name")]
    pub to_name: String,

    #[serde(rename = "5. Exchange Rate")]
    pub exchange_rate: String,
}

pub async fn fetch_exchange_rate(
    client: &Client,
    from: String,
    to: String,
    amount: f32,
) -> Result<String, Box<dyn Error>> {
    let key = std::env::var("VANTAGE_KEY")?;
    let url = construct_api_url(from, to, key);

    let response = client.get(url).send().await?.text().await?;

    let data: ResponseData = serde_json::from_str(&response)?;

    let ResponseData {
        data:
            ResponseBody {
                exchange_rate,
                from_code,
                from_name,
                to_code,
                to_name,
            },
    } = data;

    let converted = amount * exchange_rate.parse::<f32>()?;

    let result = format!(
        "Exchange from {from_name} into {to_name}: \nExchange rate: {exchange_rate},\n{amount}{from_code} = {converted}{to_code}"
    );

    Ok(result)
}

fn construct_api_url(from: String, to: String, key: String) -> String {
    format!("{BASE_URL}&from_currency={from}&to_currency={to}&apikey={key}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_api_url() {
        let (from, to, key) = ("from_code", "to_code", "demo");

        assert_eq!(
            "https://www.alphavantage.co/query?function=CURRENCY_EXCHANGE_RATE&from_currency=from_code&to_currency=to_code&apikey=demo",
            construct_api_url(from.to_string(), to.to_string(), key.to_string())
        )
    }

    #[test]
    fn test_serialize_body() {
        let example_body = r#"{
        "Realtime Currency Exchange Rate": {
            "1. From_Currency Code": "USD",
            "2. From_Currency Name": "United States Dollar",
            "3. To_Currency Code": "JPY",
            "4. To_Currency Name": "Japanese Yen",
            "5. Exchange Rate": "150.64400000"
        }
    }"#
        .to_string();

        let data: ResponseData = serde_json::from_str(&example_body).unwrap();

        assert_eq!(
            data,
            ResponseData {
                data: ResponseBody {
                    exchange_rate: "150.64400000".to_string(),
                    from_code: "USD".to_string(),
                    from_name: "United States Dollar".to_string(),
                    to_code: "JPY".to_string(),
                    to_name: "Japanese Yen".to_string()
                }
            }
        )
    }
}
