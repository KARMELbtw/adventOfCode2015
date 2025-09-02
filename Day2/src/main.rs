use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;


fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let regex = Regex::new(r"(?<num1>\d+)x(?<num2>\d+)x(?<num3>\d+)").unwrap();

    let mut total_area = 0;
    let mut ribbon_length = 0;
    for line in reader.lines() {
        let line = line?.trim().to_string();
        if let Some(caps) = regex.captures(&line) {
            let l: u32 = caps["num1"].parse().unwrap();
            let w: u32 = caps["num2"].parse().unwrap();
            let h: u32 = caps["num3"].parse().unwrap();

            let mut dims = [l, w, h];
            dims.sort();

            total_area += 2*l*2 + 2*2*h + 2*h*l + dims[0]*dims[1];
            ribbon_length += 2*(dims[0]+dims[1]) + l*w*h;
        }
    }

    println!("First Answer: {}", total_area);
    println!("Second Answer: {}", ribbon_length);
    Ok(())
}
