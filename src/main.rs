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
struct Link {
    href: String,
    text: String,
}

#[derive(Debug, Serialize)]
struct Page {
    path: String,
    title: String,
    description: String,
    links: Vec<Link>,
}

#[derive(Debug, Serialize)]
struct Report {
    date: DateTime<Utc>,
    host: String,
    robots_txt_exists: bool,
    elapsed_time: std::time::Duration,

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
            elapsed_time: std::time::Duration::from_secs(0),
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
            links: vec![],
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
    let start = std::time::Instant::now();

    let mut report = Report {
        host: url.to_string(),
        ..Report::default()
    };

    get_robots_txt(url, &mut report);
    (report.main_page_exists, report.main) = get_page(url);

    let end = std::time::Instant::now();
    report.elapsed_time = end - start;

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

fn get_page(url: &str) -> (bool, Page) {
    let res = match reqwest::blocking::get(url) {
        Ok(res) => res,
        Err(err) => {
            println!("Error {}", err);
            std::process::exit(1);
        }
    };
    let mut page = Page { ..Page::default() };
    let exists = res.status() == 200;
    if !exists {
        return (exists, page);
    }

    let html = res.text().unwrap();
    let document = Html::parse_document(&html);
    let selector = Selector::parse("title").unwrap();
    for element in document.select(&selector) {
        page.title = element.inner_html();
    }

    // <meta name="description" content="Magyarul Izraelből">
    let selector = Selector::parse("meta[name='description'").unwrap();
    for element in document.select(&selector) {
        page.description = element.attr("content").unwrap().to_string();
    }
    //println!("{:?}", html);

    let selector = Selector::parse("a").unwrap();
    for element in document.select(&selector) {
        // TODO remove consequite white-space
        let text = element.text().collect::<Vec<_>>().join("");
        let text = text.split_whitespace().collect::<Vec<_>>().join(" ");

        // This is probably a hamburger or an imgae
        // TODO: report these and then decide what to do with these if there is an image as the link
        if text.is_empty() {
            continue;
        }
        match element.attr("href") {
            Some(href) => page.links.push(Link {
                href: href.to_string(),
                text,
            }),
            None => page.links.push(Link {
                href: "".to_string(),
                text,
            }),
        }
    }

    (exists, page)
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
