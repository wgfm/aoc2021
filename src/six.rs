#![allow(dead_code)]

use std::fs::File;
use std::io;

pub fn first(mut lines: io::Lines<io::BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {
    let line = lines.next().unwrap()?;

    let mut fish: [u64; 9] =
        line.split(',')
            .map(|num| num.parse().unwrap())
            .fold([0; 9], |mut acc, curr: usize| {
                acc[curr] += 1;
                acc
            });

    let n = 256;

    for _ in 0..n {
        let new_fish = fish[0];
        fish[0] = 0;

        for i in 1..9 {
            fish[i - 1] = fish[i];
        }

        fish[8] = new_fish;
        fish[6] += new_fish;
    }

    println!("{}", fish.iter().sum::<u64>());

    Ok(())
}
