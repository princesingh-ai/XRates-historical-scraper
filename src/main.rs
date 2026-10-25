use reqwest;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    println!("Hello, World!");

    let target_url = "https://www.x-rates.com/table";
    println!("Fetching data from: {}", target_url);
    let html_content = reqwest::get(target_url)
        .await?
        .text()
        .await;

    println!("Successfully downloaded {} bytes of HTML!", html_content.expect("REASON").len());
    println!("Scraping job complete.");
    Ok(())
}