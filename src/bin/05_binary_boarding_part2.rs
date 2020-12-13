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

    let mut lowest = 1024;
    let mut highest = 0;
    let mut seats_taken = vec![false; 1024];

    for line in f.lines() {
        let s = line.unwrap();

        let column = compute_column(&s[..7]);
        let row = compute_row(&s[7..]);
        let seat_id = column * 8 + row;

        seats_taken[seat_id as usize] = true;

        if seat_id < lowest {
            lowest = seat_id;
        }
        if seat_id > highest {
            highest = seat_id;
        }
    }

    let result: Vec<_> = seats_taken[lowest as usize..highest as usize]
        .iter()
        .enumerate()
        .filter(|&(_, &value)| !value)
        .map(|(index, _)| index)
        .collect();

    println!("Result: {}", result[0] as u32 + lowest);

    Ok(())
}
