#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs;
use std::io;

pub fn star_six(
    lines: io::Lines<io::BufReader<fs::File>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (oxygen, co2) = star_six_solution(lines)?;
    println!("{}", oxygen * co2);
    Ok(())
}

pub fn star_five(
    lines: io::Lines<io::BufReader<fs::File>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (gamma, epsilon) = star_five_solution(lines)?;
    println!("{}", gamma * epsilon);
    Ok(())
}

fn star_six_solution(
    lines: io::Lines<io::BufReader<fs::File>>,
) -> Result<(i32, i32), Box<dyn std::error::Error>> {
    let lines: Vec<String> = lines.map(|l| l.unwrap()).collect();

    println!("{:?}", binary_to_i32(reduce(lines.clone())));

    Ok((0, 0))
}

fn binary_to_i32(s: String) -> i32 {
    let mut result = 0;
    for (i, ch) in s.chars().enumerate() {
        if ch == '1' {
            result += 1 << (s.len() - i - 1)
        }
    }

    result
}

fn reduce(lines: Vec<String>) -> String {
    let mut next: Vec<String> = lines;

    for n in 0..next[0].len() {
        let (left, right): (Vec<String>, Vec<String>) = next
            .into_iter()
            .partition(|chs| chs.chars().nth(n) == Some('1'));

        println!("Left: {:?}\nRight: {:?}\n", left, right);

        if left.len() < right.len() {
            next = left;
        } else {
            next = right;
        }

        if next.len() == 1 {
            break;
        }
    }

    return next[0].clone();
}

fn star_five_solution(
    mut lines: io::Lines<io::BufReader<fs::File>>,
) -> Result<(i32, i32), Box<dyn std::error::Error>> {
    let line = lines.next().expect("missing first line");
    let line = line?;

    let len = line.len();

    let mut gamma: Vec<i32> = Vec::with_capacity(len);
    let mut epsilon: Vec<i32> = Vec::with_capacity(len);
    gamma.resize(len, 0);
    epsilon.resize(len, 0);

    for (i, c) in line.chars().enumerate() {
        if c == '0' {
            epsilon[i] += 1;
        } else if c == '1' {
            gamma[i] += 1;
        }
    }

    while let Some(line) = lines.next() {
        for (i, c) in line?.chars().enumerate() {
            if c == '0' {
                epsilon[i] += 1;
            } else if c == '1' {
                gamma[i] += 1;
            }
        }
    }

    gamma.reverse();
    epsilon.reverse();

    let mut gamma_result = 0;
    let mut epsilon_result = 0;

    for i in 0..gamma.len() {
        if gamma[i] > epsilon[i] {
            gamma_result += 1 << i
        } else {
            epsilon_result += 1 << i
        }
    }

    Ok((gamma_result, epsilon_result))
}

#[cfg(test)]
mod test {
    use super::*;
}
