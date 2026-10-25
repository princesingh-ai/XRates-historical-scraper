//! This file contains the structure used to store the exchange rate data that we get from the website.
//! Once the data is extracted from the HTML page, we keep it in this structure before writing everything into the CSV file.
//! Keeping the fields defined here makes it easier to access and manage the data in other parts of the program.

#[derive(Debug)]
pub struct ExchangeRate {
    // Name or code of the currency for which we got the exchange rate.
    pub currency: String,

    // The exchange rate value is stored as a string for now.
    // We can convert it to a number later if any calculations are needed.
    pub rate: String,

    // Date on which this particular exchange rate was recorded, stored in YYYY-MM-DD format.
    pub date: String,
}