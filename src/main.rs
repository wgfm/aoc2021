use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod five;
mod four;
mod one;
mod three;
mod two;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Infile expected");
        return Ok(());
    }

    let file = File::open(&args[1])?;
    let lines = BufReader::new(file).lines();

    five::first(lines)
}
