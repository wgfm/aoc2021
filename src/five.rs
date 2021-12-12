#![allow(dead_code)]

use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;
use std::{fs, io};

pub fn first(lines: io::Lines<io::BufReader<fs::File>>) -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<Line> = lines
        .map(|l| l.unwrap().parse().unwrap())
        //        .filter(|l: &Line| l.is_straight())
        .collect();

    let mut points: HashMap<(i32, i32), i32> = HashMap::new();

    for line in lines {
        if line.is_straight() {
            if line.from.y == line.to.y {
                for x in line.from.x..=line.to.x {
                    *points.entry((x, line.to.y)).or_insert(0) += 1
                }
            }
            if line.from.x == line.to.x {
                for y in line.from.y..=line.to.y {
                    *points.entry((line.to.x, y)).or_insert(0) += 1
                }
            }
        } else {
            // Implement star 2
        }
    }

    points = points.into_iter().filter(|(_k, v)| v > &1).collect();

    println!("{}", points.len());

    Ok(())
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    from: Point,
    to: Point,
}

impl Line {
    fn is_straight(&self) -> bool {
        self.from.x == self.to.x || self.from.y == self.to.y
    }
}

struct LineIterator<'a> {
    line: &'a Line,
}

impl Iterator for Line {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (one, two) = s.split_once(" -> ").unwrap();

        let (x1, y1) = one.split_once(',').unwrap();
        let (x2, y2) = two.split_once(',').unwrap();

        Ok(Line {
            from: Point {
                x: x1.parse()?,
                y: y1.parse()?,
            },
            to: Point {
                x: x2.parse()?,
                y: y2.parse()?,
            },
        })
    }
}
