use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut distances: HashMap<(String, String), i32> = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split("to").flat_map(|s| s.split("=")).map(str::trim).collect();
        distances.insert((parts[0].to_string(), parts[1].to_string()), parts[2].parse().unwrap());
    }

    let min_distance = 0;
    for i in 0..distances.len() {

    }

    println!("First Answer: {}", min_distance);

    Ok(())
}
