use std::{error::Error, fmt, fs::File, io::Write};

use html_escape::decode_html_entities_to_string;
use reqwest::blocking::get;
use scraper::{Html, Selector};

// Custom error type
#[derive(Debug)]
struct BrothError(&'static str);

impl fmt::Display for BrothError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl std::error::Error for BrothError {}

// Encapsulates the commandline args
pub struct Command {
    mode: String,
    ticker: String,
    _optional_flag: Option<String>,
}

impl Command {
    pub fn build(args: &[String]) -> Result<Command, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let mode = args[1].clone();
        let ticker = args[2].clone();

        Ok(Command {
            mode,
            ticker,
            _optional_flag: None,
        })
    }
}

pub fn run(command: Command) -> Result<(), Box<dyn Error>> {
    let url = get_summary_url_from_ticker(&command.ticker);
    let html = fetch_html(&url).unwrap();
    match command.mode.to_lowercase().as_str() {
        "quote" => println!("{}", extract_price(&html, &command.ticker)?),
        "fullname" => println!("{}", extract_full_name(&html)?),
        _ => return Err(Box::new(BrothError("invalid mode"))),
    }
    Ok(())
}

pub fn fetch_html(url: &str) -> Result<scraper::html::Html, reqwest::Error> {
    let response = get(url)?;
    let html_string = response.text()?;
    let html_object = Html::parse_document(&html_string);
    Ok(html_object)
}

pub fn extract_price(html: &scraper::html::Html, ticker: &str) -> Result<String, Box<dyn Error>> {
    let selector_string = format!(r#"fin-streamer[data-field="regularMarketPrice"][data-symbol="{}"]"#, ticker);
    let div_selector = Selector::parse(&selector_string).unwrap();

    let parsed_document: Vec<String> = html
        .select(&div_selector)
        .map(|element| element.inner_html())
        .collect();
    match parsed_document.len() > 0 {
        true => {
            let price = parsed_document.get(0).unwrap().replace("<!-- -->", "");
            Ok(price)
        }
        false => Err(Box::new(BrothError("failed to extract price from ticker"))),
    }
}

pub fn extract_full_name(html: &scraper::html::Html) -> Result<String, Box<dyn Error>> {
    let div_selector = Selector::parse(r#"h1.D\(ib\).Fz\(18px\)"#).unwrap(); 

    let parsed_document: Vec<String> = html
        .select(&div_selector)
        .map(|element| element.inner_html())
        .collect();
    match parsed_document.len() {
        1 => {
            let name = parsed_document.get(0).unwrap().to_owned();
            let mut decoded_name = String::new();
            decode_html_entities_to_string(name, &mut decoded_name);
            Ok(decoded_name)
        }
        _ => Err(Box::new(BrothError(
            "failed to extract full comany name from ticker",
        ))),
    }
}

pub fn get_summary_url_from_ticker(ticker: &str) -> String {
    let encoded_ticker = urlencoding::encode(ticker);
    let mut quote_url = "https://finance.yahoo.com/quote/".to_owned();
    quote_url.push_str(&encoded_ticker);
    quote_url
}


fn _save_to_file(html: &str, filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(html.as_bytes())?;
    Ok(())
}