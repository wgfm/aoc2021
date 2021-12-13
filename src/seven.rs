#![allow(dead_code)]

use std::fs::File;
use std::io;

// 0, 1, 1, 2, 2, 2, 4, 7, 14, 16
pub fn first(mut lines: io::Lines<io::BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut crabs: Vec<i32> = lines
        .next()
        .unwrap()?
        .split(',')
        .flat_map(|num| num.parse())
        .collect();

    let median = {
        let len = crabs.len();
        let (_, m, _) = crabs.select_nth_unstable(len / 2);
        *m
    };

    let result: i32 = crabs.iter().map(move |c| (c - median).abs()).sum();

    println!("{}", result);

    Ok(())
}

pub fn second(mut lines: io::Lines<io::BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut crabs: Vec<i32> = lines
        .next()
        .unwrap()?
        .split(',')
        .flat_map(|num| num.parse())
        .collect();

    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();

    let result = (min..=max)
        .map(|position| {
            let mut distance = 0;
            for crab in &crabs {
                let d = (crab - position).abs();
                distance += d * (d + 1) / 2;
            }
            distance
        })
        .min();

    println!("{}", result.unwrap());

    Ok(())
}
