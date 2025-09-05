use std::fs;

fn main() -> std::io::Result<()>{
    let input = fs::read_to_string("input.txt")?;
    let input = input.trim();

    let mut i = 0;

    loop {
        let candidate = format!("{}{}", input, i);
        let digest = md5::compute(candidate);
        let hash = format!("{:x}", digest);

        if hash.starts_with("00000") {
            println!("First Answer {}", i);
            break;
        }
        i += 1;
    }


    Ok(())
}
