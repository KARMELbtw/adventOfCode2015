use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use itertools::Itertools;
use regex::Regex;

fn main() -> io::Result<()>{
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let regex = Regex::new(r"^(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+).$").unwrap();
    let mut happy_levels: HashMap<(String, String), i32> = HashMap::new();
    let mut people: HashSet<String> = HashSet::new();

    for line in reader.lines() {
        let line = line?;
        if let Some(caps) = regex.captures(&line) {
            let person = caps[1].to_string();
            let sign = if &caps[2] == "gain" { 1 } else { -1 };
            let value: i32 = caps[3].parse::<i32>().unwrap() * sign;
            let neighbor = caps[4].to_string();

            happy_levels.insert((person.clone(), neighbor.clone()), value);

            people.insert(person);
            people.insert(neighbor);
        }
    }



    println!("First Answer: {}", optimise_sitting(&mut happy_levels, people.clone()));

    for human in people.clone() {
        happy_levels.insert(("me".parse().unwrap(), human.clone()), 0);
        happy_levels.insert((human.clone(),"me".parse().unwrap()), 0);
    }

    people.insert("me".parse().unwrap());


    println!("Second Answer: {}", optimise_sitting(&mut happy_levels, people));

    Ok(())
}

fn optimise_sitting(happy_levels: &mut HashMap<(String, String), i32>, people: HashSet<String>) -> i32 {
    let mut max_happiness = i32::MIN;

    for perm in people.iter().permutations(people.len()) {
        let mut total = 0;

        for i in 0..perm.len() {
            let person = perm[i];
            let neighbor = perm[(i + 1) % perm.len()];

            total += happy_levels.get(&(person.clone(), neighbor.clone())).unwrap();
            total += happy_levels.get(&(neighbor.clone(), person.clone())).unwrap();
        }

        if total > max_happiness {
            max_happiness = total;
        }
    }
    max_happiness
}
