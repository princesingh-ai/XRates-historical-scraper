//! `xrates-scraper`: A small command line program which is used to collect historical USD exchange rates.
//!
//! ```text
//! xrates-scraper
//!     First it asks the user for the starting date and ending date in YYYY-MM-DD format.
//!     Then it goes through each date and fetches the exchange rates from x-rates.com.
//!     Multiple dates are processed concurrently so that we dont have to wait for every single request to finish one by one.
//!     At the end all the collected rates are combined and saved into historical_rates_master.csv.
//! ```
//!
//! The program uses Tokio for handling multiple requests asynchronously and reqwest for sending HTTP requests.
//! A shared client is used instead of creating a new client for every date, which makes the process a bit more efficient.
//! Semaphore is used to limit the number of requests running at the same time.
//! The User-Agent header is also set so that the request looks like it is coming from a normal browser.

mod scraper;
use std::error::Error;
use std::sync::Arc;
use std::io::{self, Write};
use chrono::{NaiveDate, Days};
use reqwest::{Client, header};
use tokio::task::JoinSet;
use tokio::sync::Semaphore;

// This function takes the date from the user through the terminal.
// It keeps asking again if the entered date is not in the correct format, so the program doesnt continue with an invalid date.
// We are using NaiveDate here since we only need the date and not the time part.
fn get_date_input(prompt: &str) -> NaiveDate {
    loop {
        print!("{}", prompt);
        // Flush is needed here otherwise sometimes the prompt may not show before the user starts typing.
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Remove extra whitespace and try converting the input into a proper date.
        // If parsing fails, just display the message and let the user enter it again.
        match NaiveDate::parse_from_str(input.trim(), "%Y-%m-%d") {
            Ok(date) => return date,
            Err(_) => println!("Invalid date format. Please use YYYY-MM-DD (e.g., 2026-10-25)."),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("### X-Rates Historical Data Scraper ###");
    
    // First we get both dates from the user instead of keeping a fixed date range in the code.
    // This way the same program can be used to scrape different periods whenever required.
    let start_date = get_date_input("Enter Start Date (YYYY-MM-DD): ");
    let end_date = get_date_input("Enter End Date (YYYY-MM-DD): ");

    // There is no point in starting the scraping if the starting date comes after the ending date.
    // So in that case we show an error and stop the program here itself.
    if start_date > end_date {
        println!("Error: Start date cannot be after the end date.");
        return Ok(());
    }
    
    // Now we setup the HTTP client which will be used for all the requests.
    // The User-Agent is set to a common Chrome browser value instead of using the default reqwest one.
    // This can help with basic website compatibility checks, although it does not guarantee that the website will allow every request.
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT, 
        header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
    );

    // Build one client and reuse it for all the dates. This also allows connections to be reused rather than opening a new client every time.
    let client = Client::builder()
        .default_headers(headers)
        .build()?;
    
    // We are allowing a maximum of 10 requests to hold a semaphore permit at once.
    // This is done so that we can fetch multiple dates together without sending an unlimited number of requests to the server.
    // Arc lets all the spawned tasks share the same semaphore.
    let semaphore = Arc::new(Semaphore::new(10));
    let mut set = JoinSet::new();

    let mut current_date = start_date;
    println!("\nStarting concurrent scraping from {} to {}...", start_date, end_date);

    // Loop over the full date range, including both the start date and end date.
    // For each date we prepare the URL, send a request and parse the response if it was successful.
    // Each date is handled in its own task so that the requests can run concurrently.
    while current_date <= end_date {
        let date_str = current_date.format("%Y-%m-%d").to_string();
        let target_url = format!("https://www.x-rates.com/historical/?from=USD&amount=1&date={}", date_str);
        
        // Clone the client because each task needs its own handle to it.
        // We also acquire a permit before spawning the task, which controls how many requests can be active together.
        let client_clone = client.clone();
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        
        set.spawn(async move {
            // This will store the rates extracted for the current date.
            // Usually the parser returns the exchange rate records found in the page, but if something fails the vector stays empty.
            let mut result = Vec::new();
            
            // Send the HTTP request and check whether we got a successful response from the website.
            if let Ok(resp) = client_clone.get(&target_url).send().await {
                if resp.status().is_success() {
                    // Convert the response into HTML text and pass it to our parser.
                    // The parser is responsible for extracting the currencies and their exchange rates for this date.
                    if let Ok(html) = resp.text().await {
                        result = scraper::parse::parse_exchange_rates(&html, &date_str);
                    }
                } else {
                    // If the server returns an error status, print a warning so we know which date did not work.
                    println!("Warning: Server returned status {} for {}", resp.status(), date_str);
                }
            }
            
            // Release the permit when this task finishes, so another request can use it.
            // This also happens when the request fails and the result is empty.
            drop(permit);
            result
        });

        // Move to the next calendar day. checked_add_days avoids silently overflowing the date type.
        current_date = current_date.checked_add_days(Days::new(1)).unwrap();
    }

    // All the requests have now been spawned. We collect their results as the individual tasks finish.
    // Since tasks can finish in any order, we sort the final records later to keep the output organized by date.
    let mut all_rates = Vec::new();
    
    while let Some(res) = set.join_next().await {
        // Add the records from each completed task to the main vector.
        // A task join error is ignored here, so only successfully joined task results are included.
        if let Ok(mut daily_rates) = res {
            all_rates.append(&mut daily_rates);
        }
    }

    // Sort the complete dataset by date before exporting it.
    // This makes the CSV easier to read even though the requests were processed concurrently.
    all_rates.sort_by(|a, b| a.date.cmp(&b.date));

    // Finally write all collected records to the output CSV file.
    // The number printed here is the number of records collected, not necessarily the number of dates requested.
    println!("Scraping complete! Exporting {} records...", all_rates.len());
    scraper::export::save_to_csv(&all_rates, "historical_rates_master.csv")?;

    // Return successfully after the CSV has been saved.
    Ok(())
}