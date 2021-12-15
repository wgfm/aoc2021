#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::str::FromStr;

pub fn first(mut lines: io::Lines<io::BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut result = 0;
    while let Some(line) = lines.next() {
        let line = line?;
        let (_, output) = line.split_once(" | ").unwrap();
        let mut output = output.split(' ');
        while let Some(sig) = output.next() {
            match sig.len() {
                2 | 3 | 4 | 7 => result += 1,
                _ => {}
            }
        }
    }

    println!("{}", result);
    Ok(())
}

pub fn second(mut lines: io::Lines<io::BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut result = 0;
    while let Some(line) = lines.next() {
        let sig: Signal = line?.parse()?;
        let out_len = sig.output.len();

        let mut line_result = 0;
        for (i, out) in sig.output.iter().enumerate() {
            for (j, entry) in sig.input.iter().enumerate() {
                if entry == out {
                    line_result += (j as i64) * i64::pow(10, (out_len - i - 1).try_into().unwrap());
                }
            }
        }

        result += line_result;
    }

    println!("{}", result);
    Ok(())
}

impl FromStr for Signal {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (input, output) = s.split_once(" | ").unwrap();

        // This is probably much neater with bitmasks
        let input: Vec<HashSet<char>> = input.split(' ').map(|s| s.chars().collect()).collect();
        let one = input.iter().find(|&x| x.len() == 2).unwrap().clone();
        let four = input.iter().find(|&x| x.len() == 4).unwrap().clone();
        let seven = input.iter().find(|&x| x.len() == 3).unwrap().clone();
        let eight = input.iter().find(|&x| x.len() == 7).unwrap().clone();
        let six = input
            .iter()
            .find(|&x| x.len() == 6 && !x.is_superset(&one))
            .unwrap()
            .clone();
        let three = input
            .iter()
            .find(|&x| x.len() == 5 && x.is_superset(&one))
            .unwrap()
            .clone();
        let nine = input
            .iter()
            .find(|&x| x.len() == 6 && x.is_superset(&four))
            .unwrap()
            .clone();
        let zero = input
            .iter()
            .find(|&x| x.len() == 6 && x != &nine && x != &six)
            .unwrap()
            .clone();
        let five = input
            .iter()
            .find(|&x| x.len() == 5 && x.is_subset(&six))
            .unwrap()
            .clone();
        let two = input
            .iter()
            .find(|&x| x.len() == 5 && x != &five && x != &three)
            .unwrap()
            .clone();

        Ok(Signal {
            input: vec![zero, one, two, three, four, five, six, seven, eight, nine],
            output: output
                .split(' ')
                .map(str::chars)
                .map(|s| s.collect())
                .collect(),
        })
    }
}

struct Signal {
    input: Vec<HashSet<char>>,
    output: Vec<HashSet<char>>,
}
