use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn combination_sum(candidates: Vec<i32>, target: i32) -> (usize, usize) {
    let mut result = Vec::new();
    let mut current = Vec::new();
    let mut nums = candidates.clone();
    nums.sort();

    fn backtrack(nums: &Vec<i32>, start: usize, remaining: i32, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if remaining == 0 {
            result.push(current.clone());
            return;
        }
        for i in start..nums.len() {
            let num = nums[i];
            if num > remaining {
                break;
            }
            current.push(num);
            backtrack(nums, i + 1, remaining - num, current, result);
            current.pop();
        }
    }

    backtrack(&nums, 0, target, &mut current, &mut result);

    let min_len = result.iter().map(|v| v.len()).min().unwrap_or(0);
    let result2 = result.iter().filter(|v| v.len() == min_len).count();
    (result.len(), result2)
}


fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut capacities: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        capacities.push(line.parse::<i32>().unwrap());
    }

    let combinations = combination_sum(capacities, 150);

    println!("First Answer: {}", combinations.0);
    println!("Second Answer: {}", combinations.1);

    Ok(())
}
