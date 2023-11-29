use clap::Parser;
use regex::Regex;
use scraper::{Html, Selector};

use std::collections::{HashMap, VecDeque};

use seo_site_checker::{create_report_html, Link, Page, Report};

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

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let args = Cli::parse();
    //dbg!(&args);
    if args.verbose {
        log::info!("Welcome");
        log::info!("Processing {}", &args.host);
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

        log::info!("seen {}, now processing path: '{}'", seen.len(), path);
        if 0 < pages && pages <= seen.len() as u32 {
            log::info!("Seen {} pages. Exiting.", seen.len());
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
    std::fs::write(json_file, serialized).unwrap();
}

fn get_page(url: &str) -> (bool, Page) {
    log::info!("Processing '{}'", url);

    let mut page = Page { ..Page::default() };

    let res = match reqwest::blocking::get(url) {
        Ok(res) => res,
        Err(err) => {
            log::error!("Error {}", err);
            return (false, page);
        }
    };

    let exists = res.status() == 200;
    if !exists {
        return (exists, page);
    }

    process_html(&res.text().unwrap(), &mut page);

    (exists, page)
}

fn process_html(html: &str, page: &mut Page) {
    let document = Html::parse_document(html);
    let selector = Selector::parse("title").unwrap();
    for element in document.select(&selector) {
        page.title = element.inner_html();
    }

    // <meta name="description" content="Magyarul Izraelből">
    let selector = Selector::parse("meta[name='description'").unwrap();
    for element in document.select(&selector) {
        page.description = element.attr("content").unwrap().to_string();
    }

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
        let href = element.attr("href").unwrap_or("");
        page.links.push(Link {
            href: href.to_string(),
            text,
        });
    }
}

fn get_robots_txt(url: &str, report: &mut Report) {
    // TODO does robots.txt exist?
    // TODO parse the robots.txt and extract the links to the sitemaps
    // TODO are there sitemaps?
    let res = match reqwest::blocking::get(format!("{}/robots.txt", url)) {
        Ok(res) => res,
        Err(err) => {
            log::error!("Error fetching robots.txt {}", err);
            return;
        }
    };

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
