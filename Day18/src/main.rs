use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn main() -> io::Result<()>{
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let grid_size: i32 = 100;
    let mut input = vec![false; (grid_size * grid_size) as usize];


    for (i, line) in reader.lines().enumerate() {
        for (j, c) in line?.chars().enumerate() {
            input[j*(grid_size as usize) + i] = c == '#';
        }
    }
    let mut grid = input.clone();
    let mut new_grid = grid.clone();

    for _ in 0..100 {
        for i in 0..grid_size {
            for j in 0..grid_size {
                let i = i as i32;
                let j = j as i32;

                let mut neighbors = 0;
                for dj in -1..=1 {
                    for di in -1..=1 {
                        if di == 0 && dj == 0 {
                            continue;
                        }
                        let ni = di+i;
                        let nj = dj+j;
                        if ni >= 0 && ni < grid_size && nj >= 0 && nj < grid_size {
                            if grid[(nj*grid_size + ni) as usize] {
                                neighbors+= 1;
                            }
                        }
                    }
                }
                if grid[(j*grid_size + i) as usize] {
                    new_grid[(j*grid_size + i) as usize] = neighbors == 2 || neighbors == 3;
                } else {
                    new_grid[(j*grid_size + i) as usize] = neighbors == 3;
                }
            }
        }
        grid = new_grid.clone();
    }

    let sum1: i32 = grid.iter().map(|&b| b as i32).sum();

    println!("First Answer: {sum1}");

    grid = input.clone();
    grid[0] = true;
    grid[(grid_size-1) as usize] = true;
    grid[((grid_size-1)*grid_size) as usize] = true;
    grid[((grid_size-1)*grid_size + grid_size - 1) as usize] = true;

    for _ in 0..100 {
        for y in 0..grid_size {
            for x in 0..grid_size {
                let mut neighbors = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        if nx >= 0 && nx < grid_size && ny >= 0 && ny < grid_size {
                            if grid[(ny * grid_size + nx) as usize] {
                                neighbors += 1;
                            }
                        }
                    }
                }

                let idx = (y * grid_size + x) as usize;
                new_grid[idx] = if grid[idx] {
                    neighbors == 2 || neighbors == 3
                } else {
                    neighbors == 3
                };
            }
        }

        grid = new_grid.clone();

        grid[0] = true;
        grid[(grid_size - 1) as usize] = true;
        grid[((grid_size - 1) * grid_size) as usize] = true;
        grid[((grid_size - 1) * grid_size + grid_size - 1) as usize] = true;
    }


    let sum2: i32 = grid.iter().map(|&b| b as i32).sum();

    println!("Second Answer: {sum2}");


    Ok(())
}
