#![allow(dead_code)]

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod eight;
mod fifteen;
mod five;
mod four;
mod fourteen;
mod one;
mod seven;
mod six;
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

    eight::second(lines)
}
