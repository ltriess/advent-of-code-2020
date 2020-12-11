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

        let pos_a = &s[..i].parse::<usize>().unwrap() - 1;
        let pos_b = &s[i + 1..j - 2].parse::<usize>().unwrap() - 1;
        let character = s.chars().nth(j - 1).unwrap();
        let password = &s[j + 2..];

        let pw_at_a = password.chars().nth(pos_a).unwrap();
        let pw_at_b = password.chars().nth(pos_b).unwrap();

        if ((pw_at_a == character) & (pw_at_b != character))
            | (pw_at_a != character) & (pw_at_b == character)
        {
            counter += 1;
        }
    }

    println!("Result: {}", counter);

    Ok(())
}
