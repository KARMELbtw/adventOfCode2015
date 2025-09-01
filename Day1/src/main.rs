use std::fs;

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let mut floor = 0;
    let mut position = 1;
    let mut in_basement = false;
    for c in input.chars() {
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }
        if floor < 0 && !in_basement {
            in_basement = true;
            println!("Second answer: {}", position);
        }
        position +=1;
    }
    println!("First answer: {}", floor);
    Ok(())
}
