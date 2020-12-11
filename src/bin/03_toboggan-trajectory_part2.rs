use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn check_slopes(right: usize, down: usize) -> u64 {
    let f = File::open("data/03_input.txt").unwrap();
    let f = BufReader::new(f);

    let mut index = 0;
    let mut counter = 0;
    let mut idx = 0;

    for line in f.lines() {
        idx += 1;
        if (idx - 1) % down != 0 {
            continue;
        }

        let s = line.unwrap();
        let tree = s.chars().nth(index).unwrap() == '#';
        if tree {
            counter += 1;
        }
        index = (index + right) % s.len();
    }

    counter
}

fn main() -> io::Result<()> {
    let trees_slope_1_1 = check_slopes(1, 1);
    let trees_slope_3_1 = check_slopes(3, 1);
    let trees_slope_5_1 = check_slopes(5, 1);
    let trees_slope_7_1 = check_slopes(7, 1);
    let trees_slope_1_2 = check_slopes(1, 2);

    let result: u64 =
        trees_slope_1_1 * trees_slope_3_1 * trees_slope_5_1 * trees_slope_7_1 * trees_slope_1_2;

    println!("Result: {}", result);

    Ok(())
}
