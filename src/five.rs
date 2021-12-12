#![allow(dead_code)]

use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;
use std::{fs, io};

pub fn first(lines: io::Lines<io::BufReader<fs::File>>) -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<Line> = lines
        .map(|l| l.unwrap().parse().unwrap())
        //        .filter(|l: &Line| l.is_straight()) // remove comment for star 1
        .collect();

    println!("{:?}", lines);

    let mut points: HashMap<(i32, i32), i32> = HashMap::new();

    for line in lines {
        for Point { x, y } in line.iterator() {
            println!("Point({},{})", x, y);
            *points.entry((x, y)).or_insert(0) += 1;
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

    fn iterator(&self) -> LineIterator {
        let x_step = if self.from.x == self.to.x {
            0
        } else if self.from.x > self.to.x {
            -1
        } else {
            1
        };
        let y_step = if self.from.y == self.to.y {
            0
        } else if self.from.y > self.to.y {
            -1
        } else {
            1
        };
        LineIterator {
            line: &self,
            x: self.from.x,
            y: self.from.y,
            x_step,
            y_step,
        }
    }
}

struct LineIterator<'a> {
    line: &'a Line,
    x: i32,
    y: i32,
    x_step: i32,
    y_step: i32,
}

impl<'a> Iterator for LineIterator<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let result = Point {
            x: self.x,
            y: self.y,
        };
        self.x += self.x_step;
        self.y += self.y_step;

        // Ugly double off-by-one error :/
        if self.x == self.line.to.x + 2 * self.x_step && self.y == self.line.to.y + 2 * self.y_step
        {
            return None;
        }

        Some(result)
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
