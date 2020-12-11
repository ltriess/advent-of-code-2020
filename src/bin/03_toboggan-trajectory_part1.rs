use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let f = File::open("data/03_input.txt")?;
    let f = BufReader::new(f);

    let mut index = 0;
    let mut counter = 0;

    for line in f.lines() {
        let s = line.unwrap();
        let tree = s.chars().nth(index).unwrap() == '#';
        if tree {
            counter += 1;
        }
        index = (index + 3) % s.len();
    }

    println!("Result: {}", counter);

    Ok(())
}
