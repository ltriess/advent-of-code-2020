use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let f = File::open("data/13_input.txt")?;
    let f = BufReader::new(f);
    let lines: Vec<String> = f
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let time_start = lines[0].parse::<u32>().unwrap();
    let mut min_waiting_time = u32::MAX;
    let mut waiting_bus_id = 0;

    for bus in lines[1].split(",") {
        if bus != "x" {
            let bus_id = bus.parse::<u32>().unwrap();
            let waiting_time = bus_id - (time_start % bus_id);

            if waiting_time < min_waiting_time {
                min_waiting_time = waiting_time;
                waiting_bus_id = bus_id;
            }
        }
    }

    println!("Result: {}", waiting_bus_id * min_waiting_time);

    Ok(())
}
