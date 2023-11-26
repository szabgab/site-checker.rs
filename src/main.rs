use chrono::{DateTime, Utc};
use clap::Parser;
use scraper::{Html, Selector};
use serde::Serialize;

#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    #[arg(long, required = true)]
    host: String,

    #[arg(long, short, default_value_t = false, help = "Turn on verbose mode")]
    verbose: bool,
}

#[derive(Debug, Serialize)]
struct Page {
    path: String,
    title: String,
    description: String,
}

#[derive(Debug, Serialize)]
struct Report {
    date: DateTime<Utc>,
    host: String,
    robots_txt_exists: bool,

    main_page_exists: bool,
    main: Page,
}

#[derive(Debug, Serialize)]
struct Required {
    main_title_length: i32,
    main_description_length: i32,
}

impl Default for Report {
    fn default() -> Report {
        Report {
            date: Utc::now(),
            host: "".to_string(),
            robots_txt_exists: false,
            main_page_exists: false,
            main: Page { ..Page::default() },
        }
    }
}

impl Default for Page {
    fn default() -> Page {
        Page {
            path: "".to_string(),
            title: "".to_string(),
            description: "".to_string(),
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
    let required = Required {
        main_title_length: 10,
        main_description_length: 30,
    };

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
        "required": &required,
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
    report.main_page_exists = res.status() == 200;

    let html = res.text().unwrap();
    let document = Html::parse_document(&html);
    let selector = Selector::parse("title").unwrap();
    for element in document.select(&selector) {
        report.main.title = element.inner_html();
    }

    // <meta name="description" content="Magyarul Izraelből">
    let selector = Selector::parse("meta[name='description'").unwrap();
    for element in document.select(&selector) {
        report.main.description = element.attr("content").unwrap().to_string();
    }
    //println!("{:?}", html);
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
