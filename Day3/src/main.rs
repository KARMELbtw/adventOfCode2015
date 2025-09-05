use std::collections::HashSet;
use std::fs;

fn main() -> std::io::Result<()>{
    let input = fs::read_to_string("input.txt")?;

    let mut houses_visited = 1;
    // let mut x = 0;
    // let mut y = 0;
    let mut x1 = 0;
    let mut y1 = 0;
    let mut x2 = 0;
    let mut y2 = 0;
    let mut turn = 0;

    let mut houses_visited_cords : HashSet<(i32, i32)> = HashSet::new();
    houses_visited_cords.insert((0,0));

    for c in input.chars() {
        // match c{
        //     '<'=> x -= 1;
        //     '^'=> y += 1;
        //     '>'=> x += 1;
        //     'v'=> y -= 1;
        //     _ => {}
        // }
        // if !houses_visited_cords.contains(&(x, y)) {
        //     houses_visited += 1;
        //     houses_visited_cords.insert((x,y));
        // }

        if turn%2 == 0 {
            match c
            {
                '<'=> x1 -= 1,
                '^'=> y1 += 1,
                '>'=> x1 += 1,
                'v'=> y1 -= 1,
                _ => {}
            }
            if !houses_visited_cords.contains(&(x1, y1)) {
                houses_visited += 1;
                houses_visited_cords.insert((x1,y1));
            }
        } else {
            match c
            {
                '<'=> x2 -= 1,
                '^'=> y2 += 1,
                '>'=> x2 += 1,
                'v'=> y2 -= 1,
                _ => {}
            }
            if !houses_visited_cords.contains(&(x2, y2)) {
                houses_visited += 1;
                houses_visited_cords.insert((x2,y2));
            }
        }
        turn += 1;
    }

    // println!("First Answer: {}" ,houses_visited);
    println!("Second Answer: {}" ,houses_visited);

    Ok(())
}
