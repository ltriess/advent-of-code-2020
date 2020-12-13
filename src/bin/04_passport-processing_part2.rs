use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn get_entry<'a>(passport: &'a String, pattern: &'a str) -> &'a str {
    return if passport.contains(pattern) {
        let idx = passport.find(pattern).unwrap();
        let mut entry = &passport[idx + pattern.len()..];
        if entry.contains(" ") {
            entry = &entry[..entry.find(" ").unwrap()];
        }
        return entry;
    } else {
        ""
    };
}

fn check_byr(passport: &String) -> bool {
    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    let entry = get_entry(passport, " byr:");
    return if entry == "" {
        false
    } else {
        let year = entry.parse::<i32>().unwrap();
        (year >= 1920) & (year <= 2002)
    };
}

fn check_iyr(passport: &String) -> bool {
    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    let entry = get_entry(passport, " iyr:");
    return if entry == "" {
        false
    } else {
        let year = entry.parse::<i32>().unwrap();
        (year >= 2010) & (year <= 2020)
    };
}

fn check_eyr(passport: &String) -> bool {
    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    let entry = get_entry(passport, " eyr:");
    return if entry == "" {
        false
    } else {
        let year = entry.parse::<i32>().unwrap();
        (year >= 2020) & (year <= 2030)
    };
}

fn check_hgt(passport: &String) -> bool {
    // hgt (Height) - a number followed by either cm or in:
    //     If cm, the number must be at least 150 and at most 193.
    //     If in, the number must be at least 59 and at most 76.
    let entry = get_entry(passport, " hgt:");
    return if entry == "" {
        false
    } else {
        return if &entry[entry.len() - 2..] == "cm" {
            let height = entry[..entry.len() - 2].parse::<i32>().unwrap();
            (height >= 150) & (height <= 193)
        } else if &entry[entry.len() - 2..] == "in" {
            let height = entry[..entry.len() - 2].parse::<i32>().unwrap();
            (height >= 59) & (height <= 76)
        } else {
            false
        };
    };
}

fn check_hcl(passport: &String) -> bool {
    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    let entry = get_entry(passport, " hcl:");
    return if entry == "" {
        false
    } else {
        return if &entry[0..1] != "#" {
            false
        } else {
            let re = Regex::new(r"[a-f0-9]{6}").unwrap();
            re.is_match(entry)
        };
    };
}

fn check_ecl(passport: &String) -> bool {
    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    let entry = get_entry(passport, " ecl:");
    (entry == "amb")
        | (entry == "blu")
        | (entry == "brn")
        | (entry == "gry")
        | (entry == "grn")
        | (entry == "hzl")
        | (entry == "oth")
}

fn check_pid(passport: &String) -> bool {
    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    let entry = get_entry(passport, " pid:");
    let re = Regex::new(r"\d{9}").unwrap();
    re.is_match(entry)
}

fn is_valid(passport: String) -> bool {
    let mut valid = true;
    valid &= check_byr(&passport);
    valid &= check_iyr(&passport);
    valid &= check_eyr(&passport);
    valid &= check_hgt(&passport);
    valid &= check_hcl(&passport);
    valid &= check_ecl(&passport);
    valid &= check_pid(&passport);
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
            if is_valid(current_passport) {
                valid_counter += 1;
            }
            current_passport = "".to_owned();
        } else {
            current_passport.push_str(" ");
            current_passport.push_str(&s);
        }
    }

    if is_valid(current_passport) {
        valid_counter += 1;
    }

    println!("Result: {}", valid_counter);

    Ok(())
}
