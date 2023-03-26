use std::env;
use std::fs::File;
use std::io::Write;
use reqwest::blocking::get;
use scraper::{Html, Selector};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <URL>", args[0]);
        std::process::exit(1);
    }
    let url = &args[1];

    match scrape_html(url) {
        Ok(html) => {
            let filename = "output.html";
            match save_to_file(&html, filename) {
                Ok(_) => println!("Successfully saved HTML to {}", filename),
                Err(e) => eprintln!("Error saving HTML to file: {}", e),
            }
        }
        Err(e) => eprintln!("Error fetching URL: {}", e),
    }
}

fn scrape_html(url: &str) -> Result<String, reqwest::Error> {
    let response = get(url)?;
    let html = response.text()?;
    Ok(html)
}

fn save_to_file(html: &str, filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(html.as_bytes())?;
    Ok(())
}