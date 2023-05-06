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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let result = std::fs::read_to_string(&args.path);
    let content = match result {
        Ok(content) => { content },
        Err(error) => { return Err(error.into()); }
    };

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
