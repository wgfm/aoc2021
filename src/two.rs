use std::fs;
use std::io;

pub fn star_three(
    mut lines: io::Lines<io::BufReader<fs::File>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (x, depth) = star_three_internal(lines)?;
    println!("{}", x * depth);
    Ok(())
}

pub fn star_four(
    mut lines: io::Lines<io::BufReader<fs::File>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (x, depth) = star_four_internal(lines)?;
    println!("{}", x * depth);
    Ok(())
}

#[allow(dead_code)]
fn star_four_internal(
    mut lines: io::Lines<io::BufReader<fs::File>>,
) -> Result<(i32, i32), Box<dyn std::error::Error>> {
    let mut x = 0;
    let mut depth = 0;
    let mut aim = 0;
    while let Some(line) = lines.next() {
        let line = line?;
        let line = line.split(" ").collect::<Vec<&str>>();
        let amount = line[1].parse::<i32>()?;
        match line[0] {
            "forward" => {
                x += amount;
                depth += aim * amount;
            }
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => continue,
        }
    }

    Ok((x, depth))
}

#[allow(dead_code)]
fn star_three_internal(
    mut lines: io::Lines<io::BufReader<fs::File>>,
) -> Result<(i32, i32), Box<dyn std::error::Error>> {
    let mut x = 0;
    let mut depth = 0;
    while let Some(line) = lines.next() {
        let line = line?;
        let line = line.split(" ").collect::<Vec<&str>>();
        let amount = line[1].parse::<i32>()?;
        match line[0] {
            "forward" => x += amount,
            "down" => depth += amount,
            "up" => depth -= amount,
            _ => continue,
        }
    }

    Ok((x, depth))
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufRead;

    /*
    #[test]
    fn star_four_test() {
        let lines = io::BufReader::new(fs::File::open("data/day2.test").expect("")).lines();

        let (x, depth) = star_four(lines).expect("");
        assert_eq!(x, 15);
        assert_eq!(depth, 60);
    }

    #[test]
    fn star_three_test() {
        let lines = io::BufReader::new(fs::File::open("data/day2.test").expect("")).lines();

        let (x, depth) = star_three(lines).expect("");
        assert_eq!(x, 15);
        assert_eq!(depth, 10);
    }
    */
}
