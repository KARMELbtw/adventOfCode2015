use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut difference = 0;
    for line in reader.lines() {
        let line = line?;

        let mut chars = line.chars().peekable();
        let mut count = 0;

        chars.next();
        chars.next_back();

        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.peek() {
                    Some('\\') | Some('"') => {
                        chars.next();
                        count += 1;
                    }
                    _ => {
                        chars.next();
                        chars.next();
                        chars.next();
                        count += 1;
                    }
                }
            } else { 
                count += 1
            }
        }

        difference += line.len() - count;
    }

    println!("First Answer: {}", difference);

    Ok(())
}
