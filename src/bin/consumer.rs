use std::io::BufRead;

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    /// Phrase to prepend to received input
    prefix: String,
}

fn main() {
    let args = Args::parse();
    let stdin = std::io::stdin();

    for line in stdin.lock().lines() {
        println!("{}{}", args.prefix, line.unwrap());
    }
}
