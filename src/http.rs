use reqwest::Client;

const API_URL: &str = "https://www.alphavantage.co/query?function=CURRENCY_EXCHANGE_RATE";

pub async fn test_get(client: &Client) -> Result<(), reqwest::Error> {
    let body = client.get("https://www.thecolorapi.com/scheme?hex=0047AB&rgb=0,71,171&hsl=215,100%,34%&cmyk=100,58,0,33&format=html&mode=analogic&count=6").send().await?.text().await?;

    println!("{}", body);
    Ok(())
}

fn construct_api_url(from_currency: &str, to_currency: &str) {
    // TODO - check if currency key is correct
}
