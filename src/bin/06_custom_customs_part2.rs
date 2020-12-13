use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn get_num_questions(m: &HashMap<char, usize>, total: usize) -> usize {
    let mut counter = 0;
    for (_, v) in m {
        if v == &total {
            counter += 1;
        }
    }
    counter
}

fn main() -> io::Result<()> {
    let f = File::open("data/06_input.txt")?;
    let f = BufReader::new(f);

    let mut current_group = HashMap::new();
    let mut counter = 0;
    let mut group_size = 0;

    for line in f.lines() {
        let s = line.unwrap();

        if s == "" {
            counter += get_num_questions(&current_group, group_size);
            current_group = HashMap::new();
            group_size = 0;
        } else {
            group_size += 1;
            for c in s.chars() {
                *current_group.entry(c).or_insert(0) += 1;
            }
        }
    }

    counter += get_num_questions(&current_group, group_size);

    println!("Result: {}", counter);

    Ok(())
}
