use clap::Parser;
use std::io::BufReader;
use std::fs::File;

#[derive(Parser)]
struct Cli {
    // Pattern to search for, a string
    pattern: String,

    // File to look in, a string of a path
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }}
