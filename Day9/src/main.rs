use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

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
        let distance: i32 = parts[2].parse().unwrap();

        distances.insert((city1.clone(), city2.clone()), distance);
        distances.insert((city2.clone(), city1.clone()), distance);

        if !cities.contains(&city1) {
            cities.push(city1);
        }

        if !cities.contains(&city2) {
            cities.push(city2);
        }
    }

    let mut min_distance = i32::MAX;
    let mut max_distance = i32::MIN;

    for perm in cities.iter().permutations(cities.len()) {
        let mut total = 0;
        for i in 0..perm.len() - 1 {
            total += distances[&(perm[i].clone(), perm[i + 1].clone())];
        }

        if total < min_distance {
            min_distance = total;
        }

        if total > max_distance {
            max_distance = total;
        }
    }

    println!("First Answer: {}", min_distance);
    println!("Second Answer: {}", max_distance);

    Ok(())
}
