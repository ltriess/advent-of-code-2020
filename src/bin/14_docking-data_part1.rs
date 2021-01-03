use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn get_value_written_to_memory(bitmask: &str, value: &u64, len_bitmask: usize) -> u64 {
    assert_eq!(len_bitmask, bitmask.len());
    let value_binary = format!("{:0width$b}", value, width = len_bitmask);

    let mut result = String::from("");
    for i in 0..len_bitmask {
        let idx = i as usize;
        if bitmask.chars().nth(idx).unwrap() == 'X' {
            result.push(value_binary.chars().nth(idx).unwrap());
        } else {
            result.push(bitmask.chars().nth(idx).unwrap());
        }
    }

    u64::from_str_radix(&result, 2).unwrap()
}

fn main() -> io::Result<()> {
    let f = File::open("data/14_input.txt")?;
    let f = BufReader::new(f);

    let len_bitmask = 36;
    let mut bitmask = String::from("");
    let mut hashmap: HashMap<u64, u64> = HashMap::new();

    for line in f.lines() {
        let s = line.unwrap();

        if s.starts_with("mask") {
            bitmask = String::from(&s["mask = ".len()..]);
        } else {
            let address = s[s.chars().position(|c| c == '[').unwrap() + 1
                ..s.chars().position(|c| c == ']').unwrap()]
                .parse::<u64>()
                .unwrap();
            let value = &s[s.chars().position(|c| c == '=').unwrap() + 2..]
                .parse::<u64>()
                .unwrap();
            let value_memory = get_value_written_to_memory(&bitmask, value, len_bitmask);
            hashmap.insert(address, value_memory);
        }
    }

    let mut sum_memory = 0;
    for (_, v) in hashmap {
        sum_memory += v;
    }

    println!("Result: {}", sum_memory);

    Ok(())
}
