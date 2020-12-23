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
                let neighbors = self.neighbor_count(row as usize, column as usize);

                next[row as usize][column as usize] = match (seat, neighbors) {
                    ('L', x) if x == 0 => '#',
                    ('#', x) if x >= 4 => 'L',
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

    fn neighbor_count(&self, row: usize, column: usize) -> u8 {
        let mut count = 0;

        // Check all 8 positions.
        // nw n ne
        //  w   e
        // sw s se

        let north = row > 0;
        let south = (row as u32) < self.height - 1;
        let west = column > 0;
        let east = (column as u32) < self.width - 1;

        if north & west {
            count += match self.seats[row - 1][column - 1] {
                '#' => 1,
                _ => 0,
            }
        }

        if north {
            count += match self.seats[row - 1][column] {
                '#' => 1,
                _ => 0,
            }
        }

        if north & east {
            count += match self.seats[row - 1][column + 1] {
                '#' => 1,
                _ => 0,
            }
        }

        if west {
            count += match self.seats[row][column - 1] {
                '#' => 1,
                _ => 0,
            }
        }

        if east {
            count += match self.seats[row][column + 1] {
                '#' => 1,
                _ => 0,
            }
        }

        if south & west {
            count += match self.seats[row + 1][column - 1] {
                '#' => 1,
                _ => 0,
            }
        }

        if south {
            count += match self.seats[row + 1][column] {
                '#' => 1,
                _ => 0,
            }
        }

        if south & east {
            count += match self.seats[row + 1][column + 1] {
                '#' => 1,
                _ => 0,
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
