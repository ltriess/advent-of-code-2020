use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn turn_right(direction: char) -> char {
    return match direction {
        'N' => 'E',
        'E' => 'S',
        'S' => 'W',
        'W' => 'N',
        _ => direction,
    };
}

fn turn_left(direction: char) -> char {
    return match direction {
        'N' => 'W',
        'E' => 'N',
        'S' => 'E',
        'W' => 'S',
        _ => direction,
    };
}

fn get_movement(command: &str, direction: char) -> (i32, i32, char) {
    let mov = command.chars().nth(0).unwrap();
    let off = command[1..].parse::<i32>().unwrap();

    return if mov == 'N' {
        (-off, 0, direction)
    } else if mov == 'S' {
        (off, 0, direction)
    } else if mov == 'E' {
        (0, off, direction)
    } else if mov == 'W' {
        (0, -off, direction)
    } else if mov == 'L' {
        let turns = off / 90;
        let mut new_dir = direction;
        for _ in 0..turns {
            new_dir = turn_left(new_dir);
        }
        (0, 0, new_dir)
    } else if mov == 'R' {
        let turns = off / 90;
        let mut new_dir = direction;
        for _ in 0..turns {
            new_dir = turn_right(new_dir);
        }
        (0, 0, new_dir)
    } else {
        match direction {
            'N' => (-off, 0, direction),
            'E' => (0, off, direction),
            'S' => (off, 0, direction),
            'W' => (0, -off, direction),
            _ => (0, 0, direction),
        }
    };
}

fn main() -> io::Result<()> {
    let f = File::open("data/12_input.txt")?;
    let f = BufReader::new(f);

    let mut r = 0;
    let mut c = 0;
    let mut direction = 'E';

    for line in f.lines() {
        let update = get_movement(&line.unwrap(), direction);
        r += update.0;
        c += update.1;
        direction = update.2
    }

    println!("Result: {}", r.abs() + c.abs());

    Ok(())
}
