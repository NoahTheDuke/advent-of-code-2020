use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

// --- Day 4: Passport Processing ---
static REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

// 210
pub fn part1(input: String) -> usize {
    let mut count = 0;

    for raw_passport in input.split("\n\n") {
        let mut passport = HashMap::new();

        for fields in raw_passport.split_whitespace() {
            let pair: Vec<&str> = fields.split(":").collect();
            passport.insert(pair[0], pair[1]);
        }

        let mut valid = true;
        for rf in REQUIRED_FIELDS.iter() {
            if !passport.contains_key(rf) {
                valid = false;
                break;
            }
        }
        if valid {
            count += 1
        }
    }
    count
}

lazy_static! {
    static ref HGT: Regex = Regex::new(r"^(?P<num>\d+)(?P<unit>cm|in)$").unwrap();
    static ref HCL: Regex = Regex::new(r"^(#[0-9a-f]{6})$").unwrap();
    static ref ECL: Regex = Regex::new(r"^(amb|blu|brn|grn|gry|hzl|oth)$").unwrap();
    static ref PID: Regex = Regex::new(r"^(\d{9})$").unwrap();
}

// 131
pub fn part2(input: String) -> usize {
    let mut count = 0;

    for raw_passport in input.split("\n\n") {
        let mut passport = HashMap::new();

        for fields in raw_passport.split_whitespace() {
            let pair: Vec<&str> = fields.split(":").collect();
            passport.insert(pair[0], pair[1]);
        }

        let mut valid = true;
        for rf in REQUIRED_FIELDS.iter() {
            if !passport.contains_key(rf) {
                valid = false;
                break;
            }
            let val = passport.get(rf).unwrap().to_string();
            valid = match *rf {
                // byr (Birth Year) - four digits; at least 1920 and at most 2002.
                "byr" => match val.parse::<usize>() {
                    Ok(1920..=2002) => true,
                    _ => false,
                },
                // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
                "iyr" => match val.parse::<usize>() {
                    Ok(2010..=2020) => true,
                    _ => false,
                },
                // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
                "eyr" => match val.parse::<usize>() {
                    Ok(2020..=2030) => true,
                    _ => false,
                },
                // hgt (Height) - a number followed by either cm or in:
                //     If cm, the number must be at least 150 and at most 193.
                //     If in, the number must be at least 59 and at most 76.
                "hgt" => match HGT.captures(&val) {
                    Some(result) => {
                        let unit = result.name("unit").map_or("", |m| m.as_str());
                        let num = result
                            .name("num")
                            .map_or("", |m| m.as_str())
                            .parse::<usize>()
                            .unwrap_or(0);
                        match unit {
                            "cm" => 150 <= num && num <= 193,
                            "in" => 59 <= num && num <= 76,
                            _ => false,
                        }
                    }
                    None => false,
                },
                // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
                "hcl" => HCL.is_match(&val),
                // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
                "ecl" => ECL.is_match(&val),
                // pid (Passport ID) - a nine-digit number, including leading zeroes.
                "pid" => PID.is_match(&val),
                _ => true,
            };
            if !valid {
                break;
            }
        }
        if valid {
            count += 1
        }
    }
    count
}
