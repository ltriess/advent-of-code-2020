use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let f = File::open("data/02_input.txt")?;
    let f = BufReader::new(f);

    let mut counter = 0;

    for line in f.lines() {
        let s = line.unwrap();

        let i = s.chars().position(|c| c == '-').unwrap();
        let j = s.chars().position(|c| c == ':').unwrap();

        let minval = &s[..i].parse::<usize>().unwrap();
        let maxval = &s[i + 1..j - 2].parse::<usize>().unwrap();
        let character = s.chars().nth(j - 1).unwrap();
        let password = &s[j + 2..];

        let occurrence = password.matches(character).count();

        if (minval <= &occurrence) & (&occurrence <= maxval) {
            counter += 1;
        }
    }

    println!("Result: {}", counter);

    Ok(())
}
