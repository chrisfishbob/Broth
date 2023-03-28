use std::{error::Error, fmt, fs::File, io::Write};

use html_escape::decode_html_entities_to_string;
use reqwest::blocking::get;
use scraper::{Html, Selector};

// Custom error type
#[derive(Debug)]
struct BrothError(String);

impl fmt::Display for BrothError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for BrothError {}

// Encapsulates the commandline args
pub struct Command {
    mode: String,
    ticker: String,
    optional_flag: Option<String>,
}

impl Command {
    pub fn build(args: &[String]) -> Result<Command, &'static str> {
        if args.len() < 3 {
            print_usage_instructions();
            std::process::exit(1);
        }
        let mode = args[1].clone();
        let ticker = args[2].clone();
        let optional_flag = if args.len() == 4 {
            Some(args[3].clone())
        } else {
            None
        };

        Ok(Command {
            mode,
            ticker,
            optional_flag,
        })
    }
}

pub fn run(command: Command) -> Result<(), Box<dyn Error>> {
    let url = get_summary_url_from_ticker(&command.ticker);
    let html = fetch_html(&url).unwrap();
    match command {
        Command {
            mode,
            ticker,
            optional_flag: None,
        } => {
            let query_string = get_query_string(&mode, &ticker);
            println!("{}", scrape_element(&html, &mode, &query_string)?)
        }
        Command {
            optional_flag: Some(_),
            ..
        } => {
            return Err(Box::new(BrothError(
                "optional flags not yet supprted".to_owned(),
            )))
        }
    }
    Ok(())
}

fn get_query_string(mode: &str, ticker: &str) -> String {
    let query_string = match mode {
        "quote" => format!(
            r#"fin-streamer[data-field="regularMarketPrice"][data-symbol="{}"]"#,
            ticker
        ),
        "fullname" => r#"h1.D\(ib\).Fz\(18px\)"#.to_owned(),
        "pe" => r#"td[data-test="PE_RATIO-value"]"#.to_owned(),
        "open" => r#"td[data-test="OPEN-value"]"#.to_owned(),
        "close" => r#"td[data-test="PREV_CLOSE-value"]"#.to_owned(),
        "pricechange" => r#"fin-streamer[data-test="qsp-price-change"] span"#.to_owned(),
        "percentchange" => {
            r#"fin-streamer[data-field="regularMarketChangePercent"] span"#.to_owned()
        }
        _ => panic!("Critical error: unable to get query_string"),
    };
    query_string
}

pub fn fetch_html(url: &str) -> Result<scraper::html::Html, reqwest::Error> {
    let response = get(url)?;
    let html_string = response.text()?;
    let html_object = Html::parse_document(&html_string);
    Ok(html_object)
}

pub fn scrape_element(
    html: &scraper::html::Html,
    scrape_target: &str,
    selector_string: &str,
) -> Result<String, Box<dyn Error>> {
    let div_selector = Selector::parse(&selector_string).unwrap();

    let parsed_document: Vec<String> = html
        .select(&div_selector)
        .map(|element| element.inner_html())
        .collect();
    match parsed_document.len() > 0 {
        true => {
            let scarped_element = parsed_document.get(0).unwrap().to_owned();
            let mut decoded_element = String::new();
            decode_html_entities_to_string(scarped_element, &mut decoded_element);
            Ok(decoded_element)
        }
        false => {
            let error_message = format!(
                "Failed to scrape {scrape_target}.\nparsed_document: {:?}",
                parsed_document
            );
            Err(Box::new(BrothError(error_message)))
        }
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

fn print_usage_instructions() {
    println!("usage: broth <mode> <ticker> [<flags> ...]\n");
    println!("available modes:");
    println!("\tquote, fullname, pe, open, close, pricechange, percentchange\n");
}
