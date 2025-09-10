use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let original_instr: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let mut wires: HashMap<String,u16> = HashMap::new();
    run_circuit(&original_instr, &mut wires);
    let a_val = wires["a"];
    println!("First Answer: {}", a_val);

    let mut wires2: HashMap<String,u16> = HashMap::new();
    let mut instr2 = original_instr.clone();

    instr2.retain(|line| !line.ends_with("-> b"));
    instr2.push(format!("{} -> b", a_val));

    run_circuit(&instr2, &mut wires2);
    println!("Second Answer: {}", wires2["a"]);

    Ok(())
}

fn run_circuit(instr: &[String], wires: &mut HashMap<String,u16>) {
    let get = |s: &str, w: &HashMap<String,u16>| s.parse().ok().or_else(|| w.get(s).cloned());
    let mut instr = instr.to_vec();

    while !instr.is_empty() {
        instr.retain(|line| {
            let parts: Vec<_> = line.split_whitespace().collect();
            match parts.as_slice() {
                [v, "->", out] => get(v, wires).map(|val| { wires.insert(out.to_string(), val); }).is_none(),
                ["NOT", i, "->", o] => get(i, wires).map(|val| { wires.insert(o.to_string(), !val & 0xFFFF); }).is_none(),
                [l, op, r, "->", o] => {
                    if let (Some(a), Some(b)) = (get(l, wires), get(r, wires)) {
                        let val = match *op { "AND"=>a&b, "OR"=>a|b, "LSHIFT"=>(a<<b)&0xFFFF, "RSHIFT"=>a>>b, _=>0 };
                        wires.insert(o.to_string(), val);
                        false
                    } else { true }
                }
                _ => true
            }
        });
    }
}