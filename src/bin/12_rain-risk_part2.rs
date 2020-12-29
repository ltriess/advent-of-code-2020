use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn rotate_waypoint_right(rs: i32, cs: i32) -> (i32, i32) {
    return if (rs < 0) & (cs < 0) {
        // North-West.
        (-cs.abs(), rs.abs())
    } else if (rs < 0) & (cs >= 0) {
        // North-East.
        (cs.abs(), rs.abs())
    } else if (rs >= 0) & (cs >= 0) {
        // South-East.
        (cs.abs(), -rs.abs())
    } else if (rs >= 0) & (cs < 0) {
        // South-West.
        (-cs.abs(), -rs.abs())
    } else {
        (rs, cs)
    };
}

fn rotate_waypoint_left(rs: i32, cs: i32) -> (i32, i32) {
    return if (rs < 0) & (cs < 0) {
        // North-West.
        (cs.abs(), -rs.abs())
    } else if (rs < 0) & (cs >= 0) {
        // North-East.
        (-cs.abs(), -rs.abs())
    } else if (rs >= 0) & (cs >= 0) {
        // South-East.
        (-cs.abs(), rs.abs())
    } else if (rs >= 0) & (cs < 0) {
        // South-West.
        (cs.abs(), rs.abs())
    } else {
        (rs, cs)
    };
}

fn get_movement(command: &str, waypoint_rs: i32, waypoint_cs: i32) -> (i32, i32, i32, i32) {
    let mov = command.chars().nth(0).unwrap();
    let off = command[1..].parse::<i32>().unwrap();

    return if mov == 'N' {
        (waypoint_rs - off, waypoint_cs, 0, 0)
    } else if mov == 'S' {
        (waypoint_rs + off, waypoint_cs, 0, 0)
    } else if mov == 'E' {
        (waypoint_rs, waypoint_cs + off, 0, 0)
    } else if mov == 'W' {
        (waypoint_rs, waypoint_cs - off, 0, 0)
    } else if mov == 'L' {
        let turns = off / 90;
        let mut new_waypoint = (waypoint_rs, waypoint_cs);
        for _ in 0..turns {
            new_waypoint = rotate_waypoint_left(new_waypoint.0, new_waypoint.1);
        }
        (new_waypoint.0, new_waypoint.1, 0, 0)
    } else if mov == 'R' {
        let turns = off / 90;
        let mut new_waypoint = (waypoint_rs, waypoint_cs);
        for _ in 0..turns {
            new_waypoint = rotate_waypoint_right(new_waypoint.0, new_waypoint.1);
        }
        (new_waypoint.0, new_waypoint.1, 0, 0)
    } else {
        (
            waypoint_rs,
            waypoint_cs,
            off * waypoint_rs,
            off * waypoint_cs,
        )
    };
}

fn main() -> io::Result<()> {
    let f = File::open("data/12_input.txt")?;
    let f = BufReader::new(f);

    let mut ship_r = 0;
    let mut ship_c = 0;
    // Waypoint position relative to ship.
    let mut waypoint_rs = -1;
    let mut waypoint_cs = 10;

    for line in f.lines() {
        let update = get_movement(&line.unwrap(), waypoint_rs, waypoint_cs);
        waypoint_rs = update.0;
        waypoint_cs = update.1;
        ship_r += update.2;
        ship_c += update.3;
    }

    println!("{}, {}", ship_r, ship_c);
    println!("Result: {}", ship_r.abs() + ship_c.abs());

    Ok(())
}
