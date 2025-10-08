use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn check_aunt(vals: &[&str], message: &HashMap<&str, i32>) -> (bool, bool) {
    let mut aunt = true;
    let mut real_aunt = true;

    let get_val = |key: &str| {
      vals.iter().position(|&n| n== key).and_then(|i| vals.get(i+1)).and_then(|v| v.parse::<i32>().ok())
    };

    for &key in message.keys() {
        if let Some(val) = get_val(key) {
            let message_val = message[key];
            
            match key {
                "cats" | "trees" => {
                    if val != message_val {aunt = false}
                    if val <= message_val { real_aunt = false}
                }
                "pomeranians" | "goldfish" => {
                    if val != message_val {aunt = false}
                    if val >= message_val { real_aunt = false}
                }
                _ => {
                    if val != message_val {
                        aunt = false;
                        real_aunt = false;
                    }
                }
            }
        }
    }

    (aunt, real_aunt)
}

fn main() -> io::Result<()>{
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let mut line = line?;
        line = line.replace("Sue ", "").replace(":", "").replace(",", "");
        let vals: Vec<&str> = line.split(" ").collect();
        let message: HashMap<&str, i32> = HashMap::from([
            ("children", 3),
            ("cats", 7),
            ("samoyeds", 2),
            ("pomeranians", 3),
            ("akitas", 0),
            ("vizslas", 0),
            ("goldfish", 5),
            ("trees", 3),
            ("cars", 2),
            ("perfumes", 1),
        ]);

        let (aunt, real_aunt) = check_aunt(&vals, &message);

        if aunt {
            println!("First Answer: {}", vals[0].parse::<i32>().unwrap());
        }

        if real_aunt {
            println!("Second Answer: {}", vals[0].parse::<i32>().unwrap());
        }

    }

    Ok(())
}
