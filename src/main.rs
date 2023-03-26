use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <ticker>", args[0]);
        std::process::exit(1);
    }
    let ticker = &args[1];
    let url = get_quote_url_from_ticker(ticker);

    match scrape_html(url.as_str()) {
        Ok(html) => {
            let div_texts = extract_data(&html);
            println!("Price: {div_texts}");
        }
        Err(e) => eprintln!("Error fetching URL: {}", e),
    }
}

fn scrape_html(url: &str) -> Result<String, reqwest::Error> {
    let response = get(url)?;
    let html = response.text()?;
    Ok(html)
}

fn extract_data(html: &str) -> String {
    let document = Html::parse_document(html);
    let div_selector = Selector::parse(r#"fin-streamer.Fw\(b\).Fz\(36px\)"#).unwrap();

    let parsed_document: Vec<String> = document
        .select(&div_selector)
        .map(|element| element.inner_html())
        .collect();
    match parsed_document.len() {
        1 => parsed_document.get(0).unwrap().to_owned(),
        _ => panic!("Parsing failed"),
    }
}

fn get_quote_url_from_ticker(ticker: &str) -> String {
    let mut quote_url = "https://finance.yahoo.com/quote/".to_owned();
    quote_url.push_str(ticker);
    quote_url.push_str("?p=");
    quote_url.push_str(ticker);

    quote_url
}
