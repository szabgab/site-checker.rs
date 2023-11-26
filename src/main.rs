use chrono::{DateTime, Utc};
use clap::Parser;
use serde::Serialize;
//use chrono::serde::ts_seconds;

// TODO create report a JSON file
// TODO from the JSON file create a report in HTML format

#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    #[arg(long, required = true)]
    host: String,

    #[arg(long, short, default_value_t = false, help = "Turn on verbose mode")]
    verbose: bool,
}

#[derive(Debug, Serialize)]
struct Report {
    //    #[serde(with = "ts_seconds")]
    date: DateTime<Utc>,
}

impl Default for Report {
    fn default() -> Report {
        Report { date: Utc::now() }
    }
}

fn main() {
    let args = Cli::parse();
    //dbg!(&args);
    if args.verbose {
        println!("Welcome!");
        println!("Processing {}", &args.host);
    }

    process(&args.host);
}

fn process(url: &str) {
    // TODO: check if URL is a root URL https://site.something.com/

    let report = Report {
        ..Report::default()
    };

    get_robots_txt(url);
    get_main_page(url);

    let serialized = serde_json::to_string(&report).unwrap();
    println!("{}", serialized);
    std::fs::write("report.json", serialized).unwrap();
}

fn get_main_page(url: &str) {
    let res = match reqwest::blocking::get(url) {
        Ok(res) => res,
        Err(err) => {
            println!("Error {}", err);
            std::process::exit(1);
        }
    };
    println!("{:?}", res.status());
    //println!("{:?}", res);
}

fn get_robots_txt(url: &str) {
    // TODO does robots.txt exist?
    // TODO parse the robots.txt and extract the links to the sitemaps
    // TODO are there sitemaps?
    let res = match reqwest::blocking::get(format!("{}robots.txt", url)) {
        Ok(res) => res,
        Err(err) => {
            println!("Error {}", err);
            std::process::exit(1);
        }
    };
    println!("{:?}", res.status());
    println!("{:?}", res.text());
}
