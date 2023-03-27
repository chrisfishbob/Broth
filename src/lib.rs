use reqwest::blocking::get;
use scraper::{Html, Selector};
use html_escape::decode_html_entities_to_string;


pub fn fetch_html(url: &str) -> Result<scraper::html::Html, reqwest::Error> {
    let response = get(url)?;
    let html_string = response.text()?;
    let html_object = Html::parse_document(&html_string);
    Ok(html_object)
}

pub fn extract_price(html: &scraper::html::Html) -> String {
    let div_selector = Selector::parse(r#"[data-test-id='symbol-price']"#).unwrap();

    let parsed_document: Vec<String> = html
        .select(&div_selector)
        .map(|element| element.inner_html())
        .collect();
    match parsed_document.len() {
        1 => parsed_document.get(0).unwrap().replace("<!-- -->", ""),
        _ => panic!("Parsing failed"),
    }
}

pub fn extract_full_name(html: &scraper::html::Html) -> String {
    let div_selector = Selector::parse(r#"[data-test-id='symbol-full-name']"#).unwrap();

    let parsed_document: Vec<String> = html
        .select(&div_selector)
        .map(|element| element.inner_html())
        .collect();
    match parsed_document.len() {
        1 => {
            let name = parsed_document.get(0).unwrap().to_owned();
            let mut decoded_name = String::new();
            decode_html_entities_to_string(name, &mut decoded_name);
            decoded_name
        }
        _ => panic!("Parsing failed"),
    }
}

pub fn get_summary_url_from_ticker(ticker: &str) -> String {
    let mut quote_url = "https://seekingalpha.com/symbol/".to_owned();
    quote_url.push_str(ticker);
    quote_url
}