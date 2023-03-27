use std::env;
use broth;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <ticker>", args[0]);
        std::process::exit(1);
    }
    let ticker = &args[1];
    let url = broth::get_summary_url_from_ticker(ticker);

    match broth::fetch_html(&url) {
        Ok(html) => {
            let full_name = broth::extract_full_name(&html);
            let price = broth::extract_price(&html);
            println!("{full_name}");
            println!("{price}");
        }
        Err(e) => eprintln!("Error fetching URL: {}", e),
    }
}