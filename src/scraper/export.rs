//! This file is responsible for saving the collected exchange rate data into a CSV file.
//! After the data has been fetched and parsed, we use this module to arrange the records into rows and write them to the local system.
//! The CSV format makes it easier to open the data in Excel or use it later for other analysis and machine learning tasks.

use std::fs;
use std::error::Error;
use super::models::ExchangeRate;

// This function takes the list of exchange rates and the name of the file where we want to save them.
// It returns a Result so that any error while writing the file can be passed back to the caller.
pub fn save_to_csv(rates: &[ExchangeRate], filename: &str) -> Result<(), Box<dyn Error>> {
    // Start with the column names so that each value in the CSV has a clear heading.
    let mut csv_content = String::from("Date,Currency,Rate\n");

    // Go through each exchange rate record and convert its fields into a comma separated row.
    // The newline at the end makes sure every record is written on a separate line.
    for item in rates {
        let row = format!("{},{},{}\n", item.date, item.currency, item.rate);
        csv_content.push_str(&row);
    }

    // Write all the generated CSV content into the specified file.
    // If the file already exists, its previous contents will be overwritten.
    // The ? operator returns the error if the file cannot be written successfully.
    fs::write(filename, csv_content)?;

    // Print a message so we know how many records were saved and where the output file is located.
    println!("Successfully saved {} records to {}", rates.len(), filename);

    // Return Ok when the file has been written without any errors.
    Ok(())
}