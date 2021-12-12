#![allow(dead_code)]
#![allow(unused_variables)]
use std::collections::HashSet;
use std::fs;
use std::io;

#[derive(Debug)]
struct Board([[(u8, bool); 5]; 5]);

impl Board {
    fn mark_number(&mut self, num: u8) -> bool {
        for row in 0..5 {
            for col in 0..5 {
                if self.0[row][col].0 == num {
                    self.0[row][col].1 = true;
                    return true;
                }
            }
        }
        false
    }

    fn has_won(&self) -> bool {
        for row in 0..5 {
            let mut col_marks = 0;
            let mut row_marks = 0;
            for col in 0..5 {
                if self.0[row][col].1 {
                    col_marks += 1;
                }
                if self.0[col][row].1 {
                    row_marks += 1;
                }

                if col_marks == 5 || row_marks == 5 {
                    return true;
                }
            }
        }

        false
    }

    fn sum_of_unmarked_numbers(&self) -> i32 {
        let mut sum = 0;
        for row in 0..5 {
            for col in 0..5 {
                if !self.0[row][col].1 {
                    sum += self.0[row][col].0 as i32;
                }
            }
        }

        return sum;
    }
}

pub fn second(
    mut lines: io::Lines<io::BufReader<fs::File>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let nums = lines.next().unwrap();
    let nums = parse_numbers(&nums?);

    let mut boards: Vec<Board> = LineToBoard::new(lines).collect();

    let board_len = boards.len();
    let mut finished_boards: HashSet<usize> = HashSet::new();

    for num in nums {
        for (i, board) in &mut boards.iter_mut().enumerate() {
            if finished_boards.contains(&i) {
                continue;
            }

            board.mark_number(num);
            if board.has_won() {
                finished_boards.insert(i);
            }

            if finished_boards.len() == board_len {
                println!("{:?}", board);
                println!(
                    "Sum of unmarked numbers: {}",
                    board.sum_of_unmarked_numbers()
                );
                println!("Called number: {}", num as i32);
                println!("Product: {}", board.sum_of_unmarked_numbers() * num as i32);
                return Ok(());
            }
        }
    }

    Ok(())
}

pub fn first(
    mut lines: io::Lines<io::BufReader<fs::File>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let nums = lines.next().unwrap();
    let nums = parse_numbers(&nums?);

    let mut boards: Vec<Board> = LineToBoard::new(lines).collect();

    for num in nums {
        for board in &mut boards {
            board.mark_number(num);
            if board.has_won() {
                println!("{:?}", board);
                println!(
                    "Sum of unmarked numbers: {}",
                    board.sum_of_unmarked_numbers()
                );
                println!("Called number: {}", num as i32);
                println!("Product: {}", board.sum_of_unmarked_numbers() * num as i32);
                return Ok(());
            }
        }
    }

    Ok(())
}

struct LineToBoard {
    lines: io::Lines<io::BufReader<fs::File>>,
}

impl LineToBoard {
    fn new(lines: io::Lines<io::BufReader<fs::File>>) -> Self {
        LineToBoard { lines }
    }
}

impl Iterator for LineToBoard {
    type Item = Board;

    fn next(&mut self) -> Option<Board> {
        if let Err(err) = self.lines.next()? {
            println!("Could not read from file: {:?}", err);
            return None;
        }

        let mut board = [[(0, false); 5]; 5];

        for row in 0..5 {
            let line = self.lines.next()?.unwrap();
            let line = line.trim();
            for (col, num) in line.split_whitespace().enumerate() {
                board[row][col].0 = num.parse::<u8>().unwrap();
            }
        }

        Some(Board(board))
    }
}

fn parse_numbers(line: &str) -> Vec<u8> {
    line.split(',').map(|n| n.parse().unwrap()).collect()
}
