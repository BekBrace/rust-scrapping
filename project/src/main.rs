use reqwest;
use scraper::{Html, Selector};
use serde::Serialize;
use std::error::Error;
use std::fs::File;
use std::io::Write;

// Struct to hold the extracted data
#[derive(Serialize)]
struct ScrapedData {
    title: String,
    link: String,
}

// Function to fetch the HTML content of a webpage
fn fetch_html(url: &str) -> Result<String, Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?;
    if !response.status().is_success() {
        return Err(format!("Failed to fetch URL: {}", response.status()).into());
    }
    let body = response.text()?;
    Ok(body)
}

// Function to extract titles and links from a webpage
fn extract_titles_and_links(html: &str) -> Vec<ScrapedData> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a").unwrap();
    let mut data = Vec::new();

    for element in document.select(&selector) {
        if let Some(title) = element.text().next() {
            if let Some(link) = element.value().attr("href") {
                data.push(ScrapedData {
                    title: title.to_string(),
                    link: link.to_string(),
                });
            }
        }
    }

    data
}

// Function to write the scraped data to a CSV file
fn write_to_csv(data: &[ScrapedData], file_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::create(file_path)?;
    let mut wtr = csv::Writer::from_writer(file);
    for record in data {
        wtr.serialize(record)?;
    }
    wtr.flush()?;
    Ok(())
}

// Function to handle pagination and collect data from multiple pages
fn scrape_multiple_pages(base_url: &str, num_pages: usize) -> Result<Vec<ScrapedData>, Box<dyn Error>> {
    let mut all_data = Vec::new();

    for page in 1..=num_pages {
        let url = format!("{}?page={}", base_url, page);
        let html = fetch_html(&url)?;
        let data = extract_titles_and_links(&html);
        all_data.extend(data);
    }

    Ok(all_data)
}

fn main() -> Result<(), Box<dyn Error>> {
    let base_url = "https://example.com/articles";
    let num_pages = 5; // Number of pages to scrape
    let file_path = "scraped_data.csv";

    let all_data = scrape_multiple_pages(base_url, num_pages)?;
    write_to_csv(&all_data, file_path)?;

    println!("Data successfully scraped and saved to {}", file_path);
    Ok(())
}
