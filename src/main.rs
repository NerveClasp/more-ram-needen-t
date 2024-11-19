use clap::Parser;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    /// Path to the file
    #[arg(short, long, value_name = "FILE")]
    file: PathBuf,
}

fn main() {
    let args = Cli::parse();

    // You can now use args.file
    println!("File path provided: {:?}", args.file);

    // Example: Check if the file exists
    if args.file.exists() {
        println!("The file exists.");
        let file = File::open(args.file).unwrap();
        let reader = BufReader::new(file);

        // HashMap to count IP occurrences
        let mut ip_counts: HashMap<String, usize> = HashMap::new();

        for line in reader.lines() {
            let line = line.unwrap();
            let re = Regex::new(r"Source IP: '(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})'");

            match re {
                Ok(re) => {
                    if let Some(captures) = re.captures(&line) {
                        if let Some(ip) = captures.get(1) {
                            let ip_str = ip.as_str().to_string();
                            *ip_counts.entry(ip_str).or_insert(0) += 1;
                        }
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        // Display the results as a table
        println!("{:<20} {}", "IP Address", "Count");
        println!("{:-<20} {}", "-", "-----");
        for (ip, count) in ip_counts {
            println!("{:<20} {}", ip, count);
        }
    } else {
        println!("The file does not exist.");
    }
}
