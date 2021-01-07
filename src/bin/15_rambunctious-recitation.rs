use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let f = File::open("data/15_input.txt")?;
    let f = BufReader::new(f);

    let numbers: Vec<u32> = f
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    let mut spoken_last = HashMap::new();
    let mut spoken_before = HashMap::new();
    for (turn, number) in numbers.iter().enumerate() {
        spoken_last.insert(number.to_owned() as usize, turn);
    }

    let mut prev = numbers[numbers.len() - 1] as usize;
    for turn in numbers.len()..2020 {
        // for part 2 use 30000000 (a bit slow, takes some seconds)
        let current;

        // Check whether number of current turn was spoken before.
        if spoken_before.contains_key(&prev) {
            current = spoken_last.get(&prev).unwrap() - spoken_before.get(&prev).unwrap();
        } else {
            current = 0;
        }

        // Update the spoken maps.
        if spoken_last.contains_key(&current) {
            spoken_before.insert(current, spoken_last.get(&current).unwrap().to_owned());
        }
        spoken_last.insert(current, turn);
        prev = current;
    }

    println!("Result: {}", prev);

    Ok(())
}
