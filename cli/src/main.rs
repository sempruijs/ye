use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Quote {
    quote: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = "https://api.kanye.rest";

    let response = reqwest::get(request_url).await?;
    let quote: Quote = response.json().await.expect("Failed here");
    println!("{}", quote.quote);

    Ok(())
}
