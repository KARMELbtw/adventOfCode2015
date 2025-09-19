use std::fs;

fn main() -> std::io::Result<()>{
    let mut input = fs::read_to_string("input.txt")?.trim().to_string();
    for i in 0..50 {
        let mut new_sequence = String::new();
        let mut chars = input.chars().peekable();

        while let Some(c) = chars.next() {
            let mut count = 1;
            while let Some(&next) = chars.peek() {
                if next == c {
                    count += 1;
                    chars.next();
                } else {
                    break;
                }

            }
            new_sequence.push_str(&count.to_string());
            new_sequence.push(c);
        }
        input = new_sequence;

        if i == 39 {
            println!("First Answer: {}", input.len());
        }
    }

    println!("Second Answer: {}", input.len());
    Ok(())
}
