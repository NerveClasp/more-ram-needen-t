use clap::Parser;
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

        for line in reader.lines() {
            let line = line.unwrap();
            println!("{}", line); // Do something with the line
        }
    } else {
        println!("The file does not exist.");
    }
}
