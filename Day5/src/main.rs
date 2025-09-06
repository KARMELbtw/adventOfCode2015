use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut nice_string_amount = 0;
    for line in reader.lines() {

        let mut previous_char = ' ';
        let mut vowel_count = 0;
        let mut has_twice_in_a_row = false;
        let mut contain_string = false;

        for c in line?.trim().chars() {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' => vowel_count += 1,
                _ => {}
            }

            if previous_char == c {
                has_twice_in_a_row = true;
            }

            match format!("{}{}", previous_char, c).as_str() {
                "ab" | "cd" | "pq" | "xy"=> contain_string = true,
                _=> {}
            }

            previous_char = c;
        }

        if vowel_count >= 3 && has_twice_in_a_row && !contain_string {
            nice_string_amount += 1;
        }
    }

    println!("First Answer: {}", nice_string_amount);
    Ok(())
}
