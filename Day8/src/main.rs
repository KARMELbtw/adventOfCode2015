use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut difference = 0;
    let mut difference2 = 0;
    for line in reader.lines() {
        let line = line?;

        let mut chars = line.chars().peekable();
        let mut count = 0;
        let mut count2 = 6;

        chars.next();
        chars.next_back();

        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.peek() {
                    Some('\\') | Some('"') => {
                        chars.next();
                        count += 1;
                        count2 += 4;
                    }
                    _ => {
                        chars.next();
                        chars.next();
                        chars.next();
                        count += 1;
                        count2 += 5;
                    }
                }
            } else { 
                count += 1;
                count2 += 1;
            }
        }

        difference += line.len() - count;
        difference2 += count2 - line.len();
    }

    println!("First Answer: {}", difference);
    println!("Second Answer: {}", difference2);

    Ok(())
}
