use clap::Parser;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
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

    if args.file.exists() {
        println!("The file exists.");
        let file = File::open(args.file).unwrap();
        let reader = BufReader::new(file);

        // HashMap to count IP occurrences
        let mut ip_counts: HashMap<String, usize> = HashMap::new();

        let mut line_count: u64 = 0;

        for line in reader.lines() {
            line_count += 1;
            print!("\rLines processed: {}", line_count);
            std::io::stdout().flush().unwrap();

            let line = line.unwrap();
            // change this regex if needed, make sure to have a capture group `(blah)`
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
        println!("\n{:<20} Count", "IP Address");
        println!("{:-<20} -----", "-");
        for (ip, count) in ip_counts {
            println!("{:<20} {}", ip, count);
        }
    } else {
        println!("The file does not exist.");
    }
}
