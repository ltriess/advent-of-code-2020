use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn compute_column(s: &str) -> u32 {
    let s_bin: String = s
        .chars()
        .map(|x| match x {
            'F' => '0',
            'B' => '1',
            _ => x,
        })
        .collect();
    u32::from_str_radix(&s_bin, 2).unwrap()
}

fn compute_row(s: &str) -> u32 {
    let s_bin: String = s
        .chars()
        .map(|x| match x {
            'L' => '0',
            'R' => '1',
            _ => x,
        })
        .collect();
    u32::from_str_radix(&s_bin, 2).unwrap()
}

fn main() -> io::Result<()> {
    let f = File::open("data/05_input.txt")?;
    let f = BufReader::new(f);

    let mut highest = 0;

    for line in f.lines() {
        let s = line.unwrap();

        let column = compute_column(&s[..7]);
        let row = compute_row(&s[7..]);
        let seat_id = column * 8 + row;
        if seat_id > highest {
            highest = seat_id;
        }
    }

    println!("Result: {}", highest);

    Ok(())
}
