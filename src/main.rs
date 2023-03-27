use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::env;
use html_escape::decode_html_entities_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <ticker>", args[0]);
        std::process::exit(1);
    }
    let ticker = &args[1];
    let url = get_summary_url_from_ticker(ticker);

    match scrape_html(&url) {
        Ok(html) => {
            let full_name = extract_full_name(&html);
            let price = extract_price(&html);
            println!("{full_name}");
            println!("Price: {price}");
        }
        Err(e) => eprintln!("Error fetching URL: {}", e),
    }
}

fn scrape_html(url: &str) -> Result<String, reqwest::Error> {
    let response = get(url)?;
    let html = response.text()?;
    Ok(html)
}

fn extract_price(html: &str) -> String {
    let document = Html::parse_document(html);
    let div_selector = Selector::parse(r#"[data-test-id='symbol-price']"#).unwrap();

    let parsed_document: Vec<String> = document
        .select(&div_selector)
        .map(|element| element.inner_html())
        .collect();
    match parsed_document.len() {
        1 => parsed_document.get(0).unwrap().replace("<!-- -->", ""),
        _ => panic!("Parsing failed"),
    }
}

fn extract_full_name(html: &str) -> String {
    let document = Html::parse_document(html);
    let div_selector = Selector::parse(r#"[data-test-id='symbol-full-name']"#).unwrap();

    let parsed_document: Vec<String> = document
        .select(&div_selector)
        .map(|element| element.inner_html())
        .collect();
    match parsed_document.len() {
        1 => {
            let price = parsed_document.get(0).unwrap().to_owned();
            let mut decoded_price = String::new();
            decode_html_entities_to_string(price, &mut decoded_price);
            decoded_price
        }
        _ => panic!("Parsing failed"),
    }
}

fn get_summary_url_from_ticker(ticker: &str) -> String {
    let mut quote_url = "https://seekingalpha.com/symbol/".to_owned();
    quote_url.push_str(ticker);

    quote_url
}
