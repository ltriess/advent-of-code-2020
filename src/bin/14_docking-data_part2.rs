use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn get_addresses_written_to(bitmask: &str, address: &u64, len_bitmask: usize) -> Vec<u64> {
    assert_eq!(len_bitmask, bitmask.len());
    let address_binary = format!("{:0width$b}", address, width = len_bitmask);

    let mut result = String::from("");
    for i in 0..len_bitmask {
        let idx = i as usize;
        if bitmask.chars().nth(idx).unwrap() == '0' {
            result.push(address_binary.chars().nth(idx).unwrap());
        } else {
            result.push(bitmask.chars().nth(idx).unwrap());
        }
    }

    let floating_bits = result.matches('X').count();
    let positions: Vec<usize> = result
        .match_indices('X')
        .collect::<Vec<(usize, &str)>>()
        .iter()
        .map(|c| c.0)
        .collect();
    let combinations = 2_u64.pow(floating_bits as u32);

    let mut addresses = vec![];

    for num in 0..combinations {
        let binary = format!("{:0width$b}", num, width = floating_bits);
        let mut current_address = result.to_owned();
        for i in 0..floating_bits {
            let mut tmp = String::from(&current_address[..positions[i]]);
            tmp.push(binary.chars().nth(i).unwrap());
            if positions[i] < len_bitmask {
                tmp.push_str(&current_address[positions[i] + 1..]);
            }
            current_address = tmp;
        }
        addresses.push(u64::from_str_radix(&current_address, 2).unwrap());
    }

    addresses
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
            let address = &s[s.chars().position(|c| c == '[').unwrap() + 1
                ..s.chars().position(|c| c == ']').unwrap()]
                .parse::<u64>()
                .unwrap();
            let value = s[s.chars().position(|c| c == '=').unwrap() + 2..]
                .parse::<u64>()
                .unwrap();
            let addresses = get_addresses_written_to(&bitmask, address, len_bitmask);
            for address in addresses {
                hashmap.insert(address, value);
            }
        }
    }

    let mut sum_memory = 0;
    for (_, v) in hashmap {
        sum_memory += v;
    }

    println!("Result: {}", sum_memory);

    Ok(())
}
