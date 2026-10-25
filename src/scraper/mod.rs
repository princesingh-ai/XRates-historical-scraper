//! This is the main module file for the scraping part of the project.
//! It connects all the different modules used in the program, like fetching the webpage, extracting exchange rates, storing the data and exporting it to a CSV file.
//! By keeping these modules separate, the code is easier to manage and each part can handle its own task.

 // Module for downloading the HTML content from the website.
pub mod fetch;

// Module which reads the HTML and extracts the currency names and their exchange rates.
pub mod parse;

// Contains the structure used to store each exchange rate along with its date.
pub mod models;

// Handles saving the collected exchange rate data into the CSV output file.
pub mod export;