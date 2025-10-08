use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use regex::Regex;

#[derive(Debug)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn combinations(total: i32, n: usize) -> Vec<Vec<i32>> {
    if n == 1 {
        return vec![vec![total]];
    }

    let mut result = Vec::new();

    for i in 0..=total {
        for mut rest in combinations(total - i, n - 1) {
            rest.insert(0, i);
            result.push(rest);
        }
    }

    result
}

fn score(ingredients: &[Ingredient], amounts: &[i32]) -> (i32, i32) {
    let mut capacity = 0;
    let mut durability = 0;
    let mut flavor = 0;
    let mut texture = 0;
    let mut calories = 0;

    for (ing, &amt) in ingredients.iter().zip(amounts.iter()) {
        capacity += ing.capacity * amt;
        durability += ing.durability * amt;
        flavor += ing.flavor * amt;
        texture += ing.texture * amt;
        calories += ing.calories * amt;
    }

    capacity = capacity.max(0);
    durability = durability.max(0);
    flavor = flavor.max(0);
    texture = texture.max(0);

    (capacity * durability * flavor * texture, calories)
}

fn main() -> io::Result<()>{
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let regex = Regex::new(r"-?\d+").unwrap();

    let mut ingredients: Vec<Ingredient> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = regex.find_iter(&*line).map(|m| m.as_str().parse::<i32>().unwrap()).collect();
        ingredients.insert(0, Ingredient {capacity: numbers[0], durability: numbers[1], flavor: numbers[2], texture: numbers[3], calories: numbers[4]});
    }

    let mut max_score = 0;
    let mut max_score_calories = 0;

    let n = 4;
    let total = 100;

    let combos = combinations(total, n);

    for combo in combos {
        let s = score(&ingredients, &combo);
        if s.0 > max_score {
            max_score = s.0;
        }

        if s.0 > max_score_calories && s.1 == 500 {
            max_score_calories = s.0;
        }
    }

    println!("First Answer: {max_score}");
    println!("Second Answer: {max_score_calories}");
    Ok(())
}