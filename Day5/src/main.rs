use std::collections::{HashMap};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut nice_string_amount_part_1 = 0;
    let mut nice_string_amount_part_2 = 0;
    for line in reader.lines() {

        let mut previous_char = ' ';
        let mut previous_previous_char = ' ';
        let mut vowel_count = 0;
        let mut has_twice_in_a_row = false;
        let mut contain_string_part_1 = false;
        let mut letter_repeats = false;
        let mut pairs : HashMap<(char, char), usize> = HashMap::new();
        let mut has_pairs = false;

        for (i, c) in line?.trim().chars().enumerate() {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' => vowel_count += 1,
                _ => {}
            }

            if previous_char == c {
                has_twice_in_a_row = true;
            }

            match format!("{}{}", previous_char, c).as_str() {
                "ab" | "cd" | "pq" | "xy"=> contain_string_part_1 = true,
                _=> {}
            }

            if i > 0 {
                let pair = (previous_char, c);

                if let Some(&last_index) = pairs.get(&pair) {
                    if i - last_index > 1 {
                        has_pairs = true;
                    }
                }

                pairs.insert(pair, i);
            }

            if previous_previous_char == c {
                letter_repeats = true;
            }

            previous_previous_char = previous_char;
            previous_char = c;
        }

        if vowel_count >= 3 && has_twice_in_a_row && !contain_string_part_1 {
            nice_string_amount_part_1 += 1;
        }

        if has_pairs && letter_repeats {
            nice_string_amount_part_2 += 1;
        }
    }

    println!("First Answer: {}", nice_string_amount_part_1);
    println!("Second Answer: {}", nice_string_amount_part_2);
    Ok(())
}
