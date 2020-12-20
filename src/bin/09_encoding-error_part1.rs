use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<u64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() -> io::Result<()> {
    let entries = read(File::open("data/09_input.txt")?)?;
    let preamble = 25;

    let mut set: HashSet<u64> = entries[..preamble].iter().cloned().collect();

    for idx in preamble..entries.len() {
        let mut found = false;
        for k in &set {
            if k > &entries[idx] {
                continue;
            }
            let diff = entries[idx] - k;
            if (&diff != k) & set.contains(&diff) {
                found = true;
                break;
            }
        }
        if !found {
            println!("Result: {}", &entries[idx]);
            break;
        }
        set.remove(&entries[idx - preamble]);
        set.insert(entries[idx]);
    }

    Ok(())
}
