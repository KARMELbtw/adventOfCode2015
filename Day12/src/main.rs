use std::{fs, io};
use serde_json::Value;

fn main() -> io::Result<()>{
    let input = fs::read_to_string("input.txt")?;
    let json: Value = serde_json::from_str(&input)?;
    let sum1 = sum_json(&json, false);
    let sum2 = sum_json(&json, true);

    println!("First Answer: {}", sum1);
    println!("Second Answer: {}", sum2);

    Ok(())
}

fn sum_json(v: &Value, ignore_red: bool) -> i64 {
    match v {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(arr) => arr.iter().map(|x| sum_json(x, ignore_red)).sum(),
        Value::Object(obj) => {
            if ignore_red && obj.values().any(|v| v == "red") {
                0
            } else {
                obj.values().map(|v| sum_json(v, ignore_red)).sum()
            }
        }
        _=> 0,
    }
}
