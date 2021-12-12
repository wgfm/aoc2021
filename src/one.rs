use std::fs::File;
use std::io;

#[allow(dead_code)]
pub fn star_two(
    mut lines: io::Lines<io::BufReader<File>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf: [i32; 4] = [0; 4];

    let mut incs = 0;

    let mut i = 0;
    while let Some(line) = lines.next() {
        buf[i % 4] = line?.parse::<i32>()?;
        i += 1;

        if i < 4 {
            continue;
        }

        let mut prev = 0;
        let mut curr = 0;
        for j in i..i + 4 {
            if j == i {
                prev += buf[j % 4];
            } else if j == i + 3 {
                curr += buf[j % 4];
            } else {
                prev += buf[j % 4];
                curr += buf[j % 4];
            }
        }

        if curr > prev {
            incs += 1;
        }
    }

    println!("{}", incs);

    Ok(())
}

#[allow(dead_code)]
pub fn star_one(
    mut lines: io::Lines<io::BufReader<File>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut incs = 0;
    let mut curr = 9000;
    while let Some(line) = lines.next() {
        let next = line?.parse::<i32>()?;

        if next > curr {
            incs += 1;
        }

        curr = next;
    }

    println!("{}", incs);

    return Ok(());
}
