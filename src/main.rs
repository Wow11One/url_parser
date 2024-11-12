use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::{BufRead, BufReader};
use ukma_url_parser::{parse_url, ParsedURL, URLError};

#[derive(Parser)]
#[command(name = "URL Parser")]
#[command(about = "Parses URLs from a file and provides scheme, domain, port, path, parameters, and fragment info", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Parse { file_path: String },
    HelpInfo,
    Credits,
}

fn main() -> Result<(), URLError> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Parse { file_path } => parse_urls_from_file(file_path)?,
        Commands::HelpInfo => {
            println!("Usage: cargo run -- [COMMAND] [OPTIONS]");
            println!("\nCommands:");
            println!("  parse   Parses URLs from a file and displays their components.");
            println!("  help    Shows help information for the URL Parser.");
            println!("  credits Shows credits for the URL Parser project.");
            println!("\nOptions:");
            println!("  -f, --file <FILE> Specifies the path to the file containing URLs.");
            println!("\nExample:");
            println!("  url_parser parse -f urls.txt");
        }
        Commands::Credits => {
            println!("URL Parser CLI");
            println!("Developed by: Volodymyr Havryliuk");
            println!("Built with Rust and powered by the pest parser library.");
        }
    }

    Ok(())
}

fn parse_urls_from_file(file_path: &str) -> Result<(), URLError> {
    let file = File::open(file_path).map_err(|_| URLError::ParsingError)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let url = line.map_err(|_| URLError::ParsingError)?;
        match parse_url(&url) {
            Ok(parsed_url) => {
                println!("Original url: {}", url);
                print_parsed_url(parsed_url);
                println!();
            }
            Err(e) => eprintln!("Error parsing URL '{}': {}", url, e),
        }
    }

    Ok(())
}

fn print_parsed_url(parsed_url: ParsedURL) {
    println!("Parsed URL:");
    println!("  Scheme: {:?}", parsed_url.url_scheme);
    println!("  Domain Parts: {:?}", parsed_url.domain_parts);
    if let Some(port) = parsed_url.port {
        println!("  Port: {}", port);
    } else {
        println!("  Port: None");
    }
    println!("  Paths: {:?}", parsed_url.paths);
    println!("  Parameters: {:?}", parsed_url.parameters);
    if let Some(fragment) = parsed_url.fragment {
        println!("  Fragment: {}", fragment);
    } else {
        println!("  Fragment: None");
    }
}
