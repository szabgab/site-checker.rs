use chrono::{DateTime, Utc};
use clap::Parser;
use serde::Serialize;
//use chrono::serde::ts_seconds;

#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    #[arg(long, required = true)]
    host: String,

    #[arg(long, short, default_value_t = false, help = "Turn on verbose mode")]
    verbose: bool,
}

// #[derive(Debug, Serialize)]
// struct Page {
//     path: String,
// }

#[derive(Debug, Serialize)]
struct Report {
    //    #[serde(with = "ts_seconds")]
    date: DateTime<Utc>,
    host: String,
    robots_txt_exists: bool,

    main_page_exists: bool,
}

impl Default for Report {
    fn default() -> Report {
        Report {
            date: Utc::now(),
            host: "".to_string(),
            robots_txt_exists: false,
            main_page_exists: false,
        }
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

    let mut report = Report {
        host: url.to_string(),
        ..Report::default()
    };

    get_robots_txt(url, &mut report);
    get_main_page(url, &mut report);

    create_report_json(&report);
    create_report_html(&report);
}

fn create_report_json(report: &Report) {
    let serialized = serde_json::to_string(&report).unwrap();
    //println!("{}", serialized);
    std::fs::write("report.json", serialized).unwrap();
}

fn create_report_html(report: &Report) {
    let template = include_str!("../templates/report.html");
    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(template)
        .unwrap();

    let globals = liquid::object!({
        "description": "Report".to_string(),
        "title": "Report".to_string(),
        "url_pagepath": "https://site-checker.code-maven.com/",
        "site_name": "Report",
        "report": &report,
    });
    let output = template.render(&globals).unwrap();

    std::fs::write("report.html", output).unwrap();
}

fn get_main_page(url: &str, report: &mut Report) {
    let res = match reqwest::blocking::get(url) {
        Ok(res) => res,
        Err(err) => {
            println!("Error {}", err);
            std::process::exit(1);
        }
    };
    //println!("{:?}", res);
    report.main_page_exists = res.status() == 200;
}

fn get_robots_txt(url: &str, report: &mut Report) {
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
    //println!("{:?}", res.text());

    report.robots_txt_exists = res.status() == 200;
}
