use std::fs::File;
use std::io::{self, BufRead, BufReader};

// Fields:
// byr (Birth Year)
// iyr (Issue Year)
// eyr (Expiration Year)
// hgt (Height)
// hcl (Hair Color)
// ecl (Eye Color)
// pid (Passport ID)
// cid (Country ID) -> optional

fn is_complete(passport: String) -> bool {
    let mut valid = true;
    valid &= passport.contains(" byr:");
    valid &= passport.contains(" iyr:");
    valid &= passport.contains(" eyr:");
    valid &= passport.contains(" hgt:");
    valid &= passport.contains(" hcl:");
    valid &= passport.contains(" ecl:");
    valid &= passport.contains(" pid:");
    return valid;
}

fn main() -> io::Result<()> {
    let f = File::open("data/04_input.txt")?;
    let f = BufReader::new(f);

    let mut current_passport = "".to_owned();
    let mut valid_counter = 0;

    for line in f.lines() {
        let s = line.unwrap();

        if s == "" {
            if is_complete(current_passport) {
                valid_counter += 1;
            }
            current_passport = "".to_owned();
        } else {
            current_passport.push_str(" ");
            current_passport.push_str(&s);
        }
    }

    if is_complete(current_passport) {
        valid_counter += 1;
    }

    println!("Result: {}", valid_counter);

    Ok(())
}
