use clap::Parser;


#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    #[arg(long, required = true)]
    host: String,

    #[arg(long, short, default_value_t = false, help = "Turn on verbose mode")]
    verbose: bool,
}


fn main() {
    let args = Cli::parse();
    //dbg!(&args);
    if args.verbose {
        println!("Welcome!");
        println!("Processing {}", &args.host);
    }
}
