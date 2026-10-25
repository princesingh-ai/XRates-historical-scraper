//! This file handles the extraction of exchange rate data from the HTML we get from x-rates.com.
//! It goes through the HTML table rows, picks out the currency name and its rate, and stores the valid records in a vector.
//! Some rows might contain unwanted text or values that are not proper numbers, so we check them before adding them to the final result.

use scrape_core::Soup;
use super::models::ExchangeRate; 

// This function takes the downloaded HTML and the date for which the data was fetched.
// We pass the date separately because the HTML page contains the rates, but we also need to know which day each record belongs to.
pub fn parse_exchange_rates(html: &str, current_date: &str) -> Vec<ExchangeRate> {
    // Create an empty vector where all the valid exchange rate records will be stored.
    let mut results = Vec::new();

    // Parse the raw HTML into a structure that we can search through more easily.
    let soup = Soup::parse(html);
    
    // Select all the table rows from the page, since the currency rates are present inside HTML table elements.
    if let Ok(rows) = soup.select("tr") {
        for row in rows {
            // Each row can have multiple cells, so we first get the td elements from the current row.
            if let Ok(cells) = row.select("td") { 
                // We need at least two cells to read the currency and its exchange rate.
                // Rows with fewer cells are skipped because they dont have the data we need.
                if cells.len() >= 2 {
                    let currency = cells[0].text().trim().to_string();
                    let rate = cells[1].text().trim().to_string();
                    
                    // Check that the rate can be parsed as a floating point number and the currency name is not empty.
                    // This helps us avoid adding table headings, blank values or other unwanted content to the dataset.
                    if rate.parse::<f64>().is_ok() && !currency.is_empty() {
                        results.push(ExchangeRate { 
                            // Attach the date passed to this function so we can identify when this rate was collected.
                            date: current_date.to_string(),
                            currency, 
                            rate 
                        });
                    }
                }
            }
        }
    }

    // Return all the records that passed the checks. If nothing valid was found, the vector will simply be empty.
    results
}