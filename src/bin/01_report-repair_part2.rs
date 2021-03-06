use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<u32>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() -> io::Result<()> {
    let entries = read(File::open("data/01_input.txt")?)?;

    let target: u32 = 2020;
    let set: HashSet<u32> = entries.iter().cloned().collect();

    'outer: for n in &entries {
        let rest = target - n;

        for m in &entries {
            if m > &rest {
                continue;
            };
            let diff = rest - m;

            if set.contains(&diff) {
                println!("Result: {}", n * m * diff);
                break 'outer;
            }
        }
    }

    Ok(())
}
