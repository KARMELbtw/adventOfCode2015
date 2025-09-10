use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() -> io::Result<()>{
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let regex = Regex::new(r"^(?:(\d+|[a-z]+)|NOT (\d+|[a-z]+)|(\d+|[a-z]+) (AND|OR|LSHIFT|RSHIFT) (\d+|[a-z]+)) -> ([a-z]+)$").unwrap();
    let mut wires : HashMap<String, i16> = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        if let Some(caps) = regex.captures(&line) {
            let output_wire = caps.get(6).unwrap().as_str().to_string();
            if let Some(assign) = caps.get(1) {
                if let Ok(num) = assign.as_str().parse::<i16>() {
                    wires.insert(output_wire, num);
                }
            } else if let Some(unary) = caps.get(2) {
                println!("  unary: NOT {} -> {}", unary.as_str(), &caps[6]);
            }
            if let Some(left) = caps.get(3) {
                let op = caps.get(4).unwrap().as_str();
                let right = caps.get(5).unwrap().as_str();
                println!("  binary: {} {} {} -> {}", left.as_str(), op, right, &caps[6]);
            }
        }
    }

    Ok(())
}
