use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let f = File::open("data/08_input.txt")?;
    let f = BufReader::new(f);
    let lines: Vec<String> = f
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let mut accumulator = 0;
    let mut pointer: i32 = 0;
    let mut visited = vec![false; lines.len()];

    loop {
        if visited[pointer as usize] {
            break;
        }

        let command = &lines[pointer as usize][..3];
        let factor = &lines[pointer as usize][4..].parse::<i32>().unwrap();
        visited[pointer as usize] = true;

        if command == "acc" {
            accumulator += factor;
            pointer += 1;
        } else if command == "jmp" {
            pointer += factor;
        } else {
            pointer += 1;
        }

        if pointer as usize == lines.len() {
            break;
        }
    }

    println!("Result: {}", accumulator);

    Ok(())
}
