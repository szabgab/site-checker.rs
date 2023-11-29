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
}
