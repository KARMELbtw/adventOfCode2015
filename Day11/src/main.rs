use std::{fs, io};

fn main() -> io::Result<()>{
    let input = fs::read_to_string("input.txt")?;
    let mut password: Vec<char> = input.trim().chars().collect();

    find_next_valid(&mut password);
    println!("First Answer: {}", password.iter().collect::<String>());

    find_next_valid(&mut password);
    println!("Second Answer: {}", password.iter().collect::<String>());

    Ok(())
}

fn find_next_valid(password: &mut Vec<char>) {
    loop {
        increment(password);
        if is_valid(password) {
            break;
        }
    }
}

fn is_valid(password: &[char]) -> bool {
    if password.contains(&'i') || password.contains(&'o') || password.contains(&'l') {
        return false;
    }

    let mut straight = false;
    for w in password.windows(3) {
        if (w[0] as u8 + 1 == w[1] as u8) && (w[1] as u8 + 1 == w[2] as u8) {
            straight = true;
            break;
        }
    }
    if !straight {
        return false;
    }

    let mut pair_count = 0;
    let mut i = 0;
    while i + 1 < password.len() {
        if password[i] == password[i + 1] {
            pair_count += 1;
            i += 2;
        } else {
            i += 1;
        }
    }

    pair_count >= 2
}

fn increment(password: &mut Vec<char>) {
    let mut i = password.len();
    while i > 0 {
        i -= 1;
        if password[i] == 'z' {
            password[i] = 'a';
        } else {
            password[i] = ((password[i] as u8) + 1) as char;
            break;
        }
    }
}