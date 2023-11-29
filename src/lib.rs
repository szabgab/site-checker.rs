use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Link {
    pub href: String,
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Page {
    pub path: String,
    pub title: String,
    pub description: String,
    pub links: Vec<Link>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Report {
    pub date: DateTime<Utc>,
    pub host: String,
    pub robots_txt_exists: bool,
    pub elapsed_time: std::time::Duration,

    pub main_page_exists: bool,
    pub main: Page,
    pub pages: Vec<Page>,
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

pub fn create_report_html(report: &Report, html_filename: &str) {
    log::info!("create_report_html {}", html_filename);
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
