use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut distances: HashMap<(String, String), i32> = HashMap::new();
    let mut cities: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split("to").flat_map(|s| s.split("=")).map(str::trim).collect();
        let city1 = parts[0].to_string();
        let city2 = parts[1].to_string();

        distances.insert((city1.clone(), city2.clone()), parts[2].parse().unwrap());
        distances.insert((city2.clone(), city1.clone()), parts[2].parse().unwrap());

        if !cities.contains(&city1) {
            cities.push(city1);
        }

        if !cities.contains(&city2) {
            cities.push(city2);
        }



    }

    let min_distance = 0;


    println!("First Answer: {}", min_distance);

    Ok(())
}
