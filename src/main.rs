use chrono::{DateTime, Utc};
use clap::Parser;
use regex::Regex;
use scraper::{Html, Selector};
use serde::Serialize;
use std::collections::{HashMap, VecDeque};

#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    #[arg(long, required = true)]
    host: String,

    #[arg(
        long,
        default_value = "report.html",
        help = "Path to the html report file"
    )]
    html: String,

    #[arg(
        long,
        default_value = "report.json",
        help = "Path to the json report file"
    )]
    json: String,

    #[arg(long, default_value_t = 0, help = "Limit number of pages to fetch")]
    pages: u32,

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
    pages: Vec<Page>,
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
            pages: vec![],
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

    let status = process(&args, &args.host, args.pages);
    std::process::exit(status);
}

fn process(args: &Cli, url: &str, pages: u32) -> i32 {
    // check if URL is a root URL https://site.something.com/
    let re = Regex::new(r"^https://[a-z.-]+/?$").unwrap();
    match re.captures(url) {
        Some(_) => {}
        None => {
            eprintln!("Invalid URL '{}'", url);
            return 1;
        }
    };
    let url = url.trim_end_matches('/');

    let start = std::time::Instant::now();

    let mut report = Report {
        host: url.to_string(),
        ..Report::default()
    };

    get_robots_txt(url, &mut report);

    let mut seen: HashMap<String, bool> = HashMap::new();

    (report.main_page_exists, report.main) = get_page(url);
    seen.insert("/".to_string(), true);

    let mut pages_queue = get_internal_links(&report.main);

    while let Some(path) = pages_queue.pop_front() {
        if path.is_empty() {
            continue;
        }
        if seen.contains_key(&path) {
            continue;
        }

        println!("seen {}, now processing path: '{}'", seen.len(), path);
        if 0 < pages && pages <= seen.len() as u32 {
            println!("Seen {} pages. Exiting.", seen.len());
            break;
        }

        let (page_exists, page) = get_page(&format!("{}{}", url, path));
        seen.insert(path.clone(), true);
        if page_exists {
            pages_queue.append(&mut get_internal_links(&page));
            report.pages.push(page);
        }
    }

    // We need a list of unique pages that we can derive from the sitemap xml and from following the internal links of the site.
    // We also need a queue of links we extracted from pages to visit

    let end = std::time::Instant::now();
    report.elapsed_time = end - start;

    if !args.json.is_empty() {
        create_report_json(&report, &args.json);
    }
    if !args.html.is_empty() {
        create_report_html(&report, &args.html);
    }

    0
}

fn get_internal_links(page: &Page) -> VecDeque<String> {
    let pages_queue = VecDeque::from(
        page.links
            .iter()
            .filter(|link| !link.href.starts_with("https://"))
            .filter(|link| !link.href.starts_with("http://"))
            .filter(|link| !link.href.starts_with("mailto:"))
            .map(|link| link.href.clone())
            .collect::<Vec<String>>(),
    );
    pages_queue
}

fn create_report_json(report: &Report, json_file: &str) {
    let serialized = serde_json::to_string(&report).unwrap();
    //println!("{}", serialized);
    std::fs::write(json_file, serialized).unwrap();
}

fn create_report_html(report: &Report, html_filename: &str) {
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

    std::fs::write(html_filename, output).unwrap();
}

fn get_page(url: &str) -> (bool, Page) {
    println!("Processing '{}'", url);

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
    let res = match reqwest::blocking::get(format!("{}/robots.txt", url)) {
        Ok(res) => res,
        Err(err) => {
            println!("Error {}", err);
            std::process::exit(1);
        }
    };
    //println!("{:?}", res.text());

    report.robots_txt_exists = res.status() == 200;
}

#[test]
fn test_get_internal_links() {
    let page = Page {
        path: "/".to_string(),
        title: "Hello World".to_string(),
        description: "This is a description".to_string(),
        links: vec![
            Link {
                href: "/about".to_string(),
                text: "About".to_string(),
            },
            Link {
                href: "https://other.site/".to_string(),
                text: "Other site".to_string(),
            },
            Link {
                href: "http://insecure.site/".to_string(),
                text: "Insecure link".to_string(),
            },
            Link {
                href: "mailto:foo@bar.com".to_string(),
                text: "Send message".to_string(),
            },
        ],
    };
    let links = get_internal_links(&page);
    assert_eq!(links.len(), 1);
    assert_eq!(links[0], "/about");
}
