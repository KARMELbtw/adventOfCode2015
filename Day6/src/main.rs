use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let regex = Regex::new(r"(\d+),(\d+) through (\d+),(\d+)").unwrap();
    let mut grid = vec![false; 1000000];
    let idx = |x: usize, y: usize| x * 1000 + y;

    for line_result in reader.lines() {
        let line = line_result?;
        if let Some(caps) = regex.captures(&line) {
            let x1: usize = caps[1].parse().unwrap();
            let y1: usize = caps[2].parse().unwrap();
            let x2: usize = caps[3].parse().unwrap();
            let y2: usize = caps[4].parse().unwrap();

            for x in x1..=x2 {
                for y in y1..=y2 {
                    let i = idx(x, y);
                    if line.contains("toggle") {
                        grid[i] = !grid[i];
                    } else if line.contains("turn on") {
                        grid[i] = true;
                    } else if line.contains("turn off") {
                        grid[i] = false;
                    }
                }
            }
        }
    }

    let lights_on_amount = grid.iter().filter(|&&b| b).count();

    println!("First Answer: {}", lights_on_amount);

    Ok(())
}
