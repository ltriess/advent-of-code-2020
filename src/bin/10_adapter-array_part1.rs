use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<u64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() -> io::Result<()> {
    let mut entries = read(File::open("data/10_input.txt")?)?;
    entries.sort();

    let mut counter_1_jump = 0;
    let mut counter_3_jump = 1;
    let mut previous = 0;

    for e in entries {
        if e - previous > 3 {
            println!("According to the problem text, this case does not exist.");
            break;
        } else if e - previous == 1 {
            counter_1_jump += 1;
        } else if e - previous == 3 {
            counter_3_jump += 1
        } else {
            continue;
        }
        previous = e;
    }

    println!("Result: {}", counter_1_jump * counter_3_jump);

    Ok(())
}
