use std::fs::File;
use std::io::{self, BufRead, BufReader};

struct WaitingArea {
    width: u32,
    height: u32,
    seats: Vec<Vec<char>>,
}

impl WaitingArea {
    fn new(seats: Vec<Vec<char>>) -> WaitingArea {
        WaitingArea {
            width: seats[0].len() as u32,
            height: seats.len() as u32,
            seats,
        }
    }

    fn update(&mut self) -> bool {
        let mut equal = true;
        let mut next = self.seats.clone();

        for row in 0..self.height {
            for column in 0..self.width {
                let seat = self.seats[row as usize][column as usize];
                let neighbors = self.occupied_count(row as usize, column as usize);

                next[row as usize][column as usize] = match (seat, neighbors) {
                    ('L', x) if x == 0 => '#',
                    ('#', x) if x >= 5 => 'L',
                    (otherwise, _) => otherwise,
                };

                // Track if value changed.
                equal &= self.seats[row as usize][column as usize]
                    == next[row as usize][column as usize];
            }
        }

        self.seats = next;

        equal
    }

    fn occupied_count(&self, row: usize, column: usize) -> u8 {
        let mut count = 0;

        // Check all 8 directions.
        // nw n ne
        //  w   e
        // sw s se

        // Check north-west direction.
        count += self.traverse(row, column, -1, -1);
        // Check north direction.
        count += self.traverse(row, column, -1, 0);
        // Check north-east direction.
        count += self.traverse(row, column, -1, 1);
        // Check west direction.
        count += self.traverse(row, column, 0, -1);
        // Check east direction.
        count += self.traverse(row, column, 0, 1);
        // Check south-west direction.
        count += self.traverse(row, column, 1, -1);
        // Check south direction.
        count += self.traverse(row, column, 1, 0);
        // Check south-east direction.
        count += self.traverse(row, column, 1, 1);

        count
    }

    fn traverse(&self, row_init: usize, column_init: usize, row_step: i32, column_step: i32) -> u8 {
        assert!(row_init < self.height as usize);
        assert!(column_init < self.width as usize);
        assert!((-1 <= row_step) | (row_step <= 1));
        assert!((-1 <= column_step) | (column_step <= 1));

        let mut count = 0;

        let mut r = ((row_init as i32) + row_step) as usize;
        let mut c = ((column_init as i32) + column_step) as usize;

        let mut proceed = (r < self.height as usize) & (c < self.width as usize);
        while proceed {
            if self.seats[r][c] == '#' {
                count += 1;
                proceed = false;
            } else if self.seats[r][c] == 'L' {
                proceed = false;
            } else {
                r = ((r as i32) + row_step) as usize;
                c = ((c as i32) + column_step) as usize;
                proceed = (r < self.height as usize) & (c < self.width as usize);
            }
        }

        count
    }

    fn occupied_seats(&self) -> u32 {
        let mut count = 0;

        for row in 0..self.height {
            for column in 0..self.width {
                if self.seats[row as usize][column as usize] == '#' {
                    count += 1;
                }
            }
        }

        count
    }
}

fn main() -> io::Result<()> {
    let f = File::open("data/11_input.txt")?;
    let f = BufReader::new(f);
    let lines: Vec<Vec<char>> = f
        .lines()
        .map(|l| l.expect("Could not parse line").chars().collect())
        .collect();

    let mut waiting_area = WaitingArea::new(lines);
    while !waiting_area.update() {}

    println!("Result: {}", waiting_area.occupied_seats());

    Ok(())
}
