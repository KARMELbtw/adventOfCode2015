use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use regex::Regex;

struct Reindeer {
    name: String,
    speed: i32,
    fly: i32,
    rest: i32,
}

impl Reindeer {
    fn calc_distance(&self, time: i32) -> i32 {
        let mut distance = 0;
        let mut flying_time = 0;
        let mut resting_time = 0;
        let mut resting = false;
        for _ in 0..time {
            if !resting {
                distance += self.speed;
                flying_time += 1;
                if flying_time == self.fly {
                    resting = true;
                    flying_time = 0;
                }
            } else {
                resting_time += 1;
                if resting_time == self.rest {
                    resting = false;
                    resting_time = 0;
                }
            }
        }
        distance
    }

    pub fn new(name: String, speed: i32, fly: i32, rest: i32) -> Self {
        Self { name, speed, fly, rest }
    }
}

fn main() -> io::Result<()>{
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let regex = Regex::new(r"(\d+)").unwrap();

    let mut reindeers : Vec<Reindeer> = vec![];
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(' ').collect();
        let name = parts[0];
        let numbers: Vec<i32> = regex.captures_iter(line.as_str()).map(|cap| cap[1].parse::<i32>().unwrap()).collect();
        reindeers.push(Reindeer::new(name.parse().unwrap(), numbers[0], numbers[1], numbers[2]));
    }

    let max_distance = reindeers.iter().map(|reindeer| reindeer.calc_distance(2503)).max().unwrap();

    let mut points: HashMap<String, i32> = HashMap::new();

    for i in 1..2504 {
        let max = reindeers.iter().map(|r| r.calc_distance(i)).max().unwrap();

        for r in reindeers.iter().filter(|r| r.calc_distance(i) == max) {
            *points.entry(r.name.clone()).or_insert(0) += 1;
        }
    }

    let max_points = points.iter().max_by_key(|entry| entry.1).unwrap().1;

    println!("First Answer: {max_distance}");
    println!("Second Answer: {max_points}");

    Ok(())
}
