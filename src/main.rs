mod http;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    http::test_get(&client).await?;
    println!("Hello, world!");
    http::test_get(&client).await?;
    Ok(())
}
