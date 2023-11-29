use seo_site_checker::{create_report_html, Report};
use std::fs::File;

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    if argv.len() < 3 || 4 < argv.len() {
        eprintln!("Usage: {} report.json report.html", &argv[0]);
        std::process::exit(1);
    }
    let json_file = &argv[1];
    let html_file = &argv[2];
    let config_file = if argv.len() == 4 { &argv[3] } else { "" };

    print!("From '{}' to '{}'", json_file, html_file);
    if config_file.is_empty() {
        println!(" using the default configuration.")
    } else {
        println!(" using the configuration from '{}'.", config_file);
    }

    let report: Report = match File::open(json_file) {
        Ok(file) => match serde_json::from_reader(file) {
            Ok(data) => data,
            Err(err) => {
                eprintln!("There was an error parsing the YAML file {}", err);
                std::process::exit(1);
            }
        },
        Err(error) => {
            eprintln!("Error opening file {}: {}", json_file, error);
            std::process::exit(1);
        }
    };

    create_report_html(&report, html_file);
}
