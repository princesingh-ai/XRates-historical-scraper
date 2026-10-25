# XRates Historical Scraper

A command-line tool written in Rust to collect historical USD exchange rates from [x-rates.com](https://www.x-rates.com/) and export the extracted data to a CSV file.

This project was built as a practical way to learn Rust, with a focus on asynchronous programming, ownership, error handling, modular code organization, and processing data from HTML pages.

## Features

- **Asynchronous Requests:** Uses Tokio and `JoinSet` to fetch exchange rate data for multiple dates concurrently.
- **Concurrency Control:** Uses a semaphore to limit the number of requests running at the same time.
- **HTML Parsing:** Uses `scrape-core` to extract currency names and exchange rates from HTML table rows.
- **Basic Data Validation:** Checks that currency names are not empty and that extracted rates can be parsed as floating-point numbers.
- **CSV Export:** Combines the extracted records, sorts them by date, and saves them to `historical_rates_master.csv`.
- **Date Range Input:** Lets users specify a start date and an end date through the command line.

## Prerequisites

- [Rust and Cargo](https://www.rust-lang.org/tools/install)
- Rust 1.70 or later is recommended, subject to the minimum versions required by the project's dependencies.

## Installation

Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/princesingh-ai/xrates-scraper-rust.git
cd xrates-scraper-rust
```

Build the project in release mode:

```bash
cargo build --release
```

## Usage

Run the application using Cargo:

```bash
cargo run --release
```

The program will prompt you to enter a start date and an end date in `YYYY-MM-DD` format.

For example:

```text
Enter Start Date (YYYY-MM-DD): 2026-01-01
Enter End Date (YYYY-MM-DD): 2026-01-07
```

The scraper then requests data for each date, processes the returned HTML, and collects the valid exchange rate records.

Once processing is complete, the results are written to:

```text
historical_rates_master.csv
```

The output file is created in the current working directory.

## Project Structure

The project separates the command-line interface, HTTP requests, HTML parsing, data structures, and CSV export into different modules.

```text
src/
├── main.rs
└── scraper/
    ├── mod.rs
    ├── fetch.rs
    ├── models.rs
    ├── parse.rs
    └── export.rs
```

- `main.rs` — Handles date input, creates the HTTP client, coordinates concurrent tasks, and collects the results.
- `fetch.rs` — Contains a helper function for downloading HTML content.
- `models.rs` — Defines the `ExchangeRate` data structure.
- `parse.rs` — Extracts currency names and rates from HTML table rows and performs basic validation.
- `export.rs` — Writes the collected records to a CSV file.
- `mod.rs` — Declares the scraper modules.

## Documentation

Generate the project's Rust documentation locally with:

```bash
cargo doc --open
```

This generates documentation from the source code's documentation comments and opens it in your browser.

## Limitations

- The scraper depends on the HTML structure of x-rates.com, which may change over time.
- Requests can fail, and the current implementation does not retry failed requests.
- The extracted records receive basic validation, but the program does not independently verify the accuracy of the exchange rates.
- The scraper's concurrency limit controls simultaneous tasks, but does not guarantee that the website will accept every request.

