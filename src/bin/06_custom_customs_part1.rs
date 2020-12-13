use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn get_num_questions(s: &str) -> usize {
    let hash_set: HashSet<char> = s.chars().into_iter().collect();
    hash_set.len()
}

fn main() -> io::Result<()> {
    let f = File::open("data/06_input.txt")?;
    let f = BufReader::new(f);

    let mut current_group = "".to_owned();
    let mut counter = 0;

    for line in f.lines() {
        let s = line.unwrap();

        if s == "" {
            counter += get_num_questions(&current_group);
            current_group = "".to_owned();
        } else {
            current_group.push_str(&s);
        }
    }

    counter += get_num_questions(&current_group);

    println!("Result: {}", counter);

    Ok(())
}
