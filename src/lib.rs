mod broth_error;
use std::{error::Error, fs::File, io::Write};

use html_escape::decode_html_entities_to_string;
use reqwest::blocking::get;
use scraper::{Html, Selector};
use termion::{clear::AfterCursor, cursor::Up};
use colored::{self, Colorize};


const SUMMARY_ELEMENTS: [&str; 7] = [
    "fullname",
    "quote",
    "pricechange",
    "percentchange",
    "open",
    "close",
    "pe",
];
const HEADER_LENGTH: usize = 2;


// Encapsulates the commandline args
pub struct Command<'a> {
    pub mode: &'a str,
    pub ticker: &'a str,
    pub optional_flag: Option<&'a str>,
}

impl<'a> Command<'a> {
    pub fn build(args: &[String]) -> Result<Command, &'static str> {
        if args.len() < 3 {
            print_usage_instructions();
            std::process::exit(1);
        }
        let mode = &args[1];
        let ticker = &args[2];
        let optional_flag = if args.len() == 4 {
            Some(args[3].as_str())
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

struct StockInfo {
    fullname: String,
    quote: String,
    pricechange: String,
    percentchange: String,
    open: String,
    close: String,
    pe: String,
}

impl StockInfo {
    fn build(html: &scraper::html::Html, ticker: &str) -> Result<Self, Box<dyn Error>> {
        let fullname = scrape_element(&html, "fullname", ticker)?;
        let quote = scrape_element(&html, "quote", ticker)?;
        let pricechange = scrape_element(&html, "pricechange", ticker)?;
        let percentchange = scrape_element(&html, "percentchange", ticker)?;
        let open = scrape_element(&html, "open", ticker)?;
        let close = scrape_element(&html, "close", ticker)?;
        let pe = scrape_element(&html, "pe", ticker).unwrap_or("N/A".to_owned());

        Ok(Self {
            fullname,
            quote,
            pricechange,
            percentchange,
            open,
            close,
            pe,
        })
    }
}

pub fn run(command: Command) -> Result<(), Box<dyn Error>> {
    let url = get_summary_url_from_ticker(&command.ticker);
    match command {
        Command {
            mode: "summary",
            ticker,
            optional_flag,
        } => loop {
            let html = fetch_html(&url).unwrap();
            let info = StockInfo::build(&html, ticker)?;
            display_summary(&info);
            match optional_flag {
                Some("--stream") => {
                    print!(
                        "{}{}",
                        Up((SUMMARY_ELEMENTS.len() + HEADER_LENGTH) as u16),
                        AfterCursor
                    )
                }
                _ => break,
            }
        },
        _ => return Err(broth_error::BrothError::new("flag not yet supported")),
    }
    Ok(())
}

fn display_summary(info: &StockInfo) {
    let price_change_float: f32 = info.pricechange.parse().unwrap();
    println!("{}", "#".repeat(info.fullname.len()));
    println!("{}", info.fullname);
    println!("{}", "#".repeat(info.fullname.len()));
    print!("quote: ");
    if price_change_float >= 0.0 {
        println!("{}", info.quote.green());
    } else {
        println!("{}", info.quote.red());
    }
    println!("pricechange: {}", info.pricechange);
    println!("percentchange: {}", info.percentchange);
    println!("open: {}", info.open);
    println!("close: {}", info.close);
    println!("pe: {}", info.pe);
}

fn get_query_string(mode: &str, ticker: &str) -> String {
    let query_string = match mode {
        "quote" => format!(
            r#"fin-streamer[data-field="regularMarketPrice"][data-symbol="{}"]"#,
            ticker.to_uppercase()
        ),
        "fullname" => r#"h1.D\(ib\).Fz\(18px\)"#.to_owned(),
        "pe" => r#"td[data-test="PE_RATIO-value"]"#.to_owned(),
        "open" => r#"td[data-test="OPEN-value"]"#.to_owned(),
        "close" => r#"td[data-test="PREV_CLOSE-value"]"#.to_owned(),
        "pricechange" => r#"fin-streamer[data-test="qsp-price-change"] span"#.to_owned(),
        "percentchange" => format!(
            r#"fin-streamer[data-field="regularMarketChangePercent"][data-symbol="{}"] span"#,
            ticker.to_uppercase()
        ),
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
    ticker: &str,
) -> Result<String, Box<dyn Error>> {
    let selector_string = get_query_string(scrape_target, ticker);
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
            Err(broth_error::BrothError::new(&error_message))
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
