use std::collections::HashSet;
use std::fs;

fn main() -> std::io::Result<()>{
    let input = fs::read_to_string("input.txt")?;

    let mut houses_visited = 1;
    let mut x = 0;
    let mut y = 0;

    let mut houses_visited_cords : HashSet<(i32, i32)> = HashSet::new();

    for c in input.chars() {
        match c{
            '<'=> {
                x -= 1;
            }
            '^'=> {
                y += 1;
            }
            '>'=> {
                x += 1;
            }
            'v'=> {
                y -= 1;
            }
            _ => {}
        }
        if !houses_visited_cords.contains(&(x, y)) {
            houses_visited += 1;
            houses_visited_cords.insert((x,y));
        }
    }

    println!("First Answer: {}" ,houses_visited);

    Ok(())
}
