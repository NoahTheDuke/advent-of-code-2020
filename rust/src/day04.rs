use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

// --- Day 4: Passport Processing ---

pub fn part1(input: String) -> String {
    let passports: Vec<&str> = input.split("\n\n").collect();
    let mut count = 0;

    let required_fields: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    for passport in passports {
        let mut temp: HashMap<String, String> = HashMap::new();

        let fields: Vec<(&str, &str)> = passport
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .map(|s| {
                let pair: Vec<&str> = s.split(":").collect();
                (pair[0], pair[1])
            })
            .collect();
        for (key, val) in fields {
            temp.insert(key.to_owned(), val.to_owned());
        }
        let mut valid = true;
        for rf in required_fields.iter() {
            if !temp.contains_key(&rf.to_string()) {
                valid = false;
                break;
            }
        }
        if valid {
            count += 1
        }
    }
    count.to_string()
}

lazy_static! {
    static ref HGT: Regex = Regex::new(r"^(?P<num>\d+)(?P<unit>cm|in)$").unwrap();
    static ref HCL: Regex = Regex::new(r"^(#[0-9a-f]{6})$").unwrap();
    static ref ECL: Regex = Regex::new(r"^(amb|blu|brn|grn|gry|hzl|oth)$").unwrap();
    static ref PID: Regex = Regex::new(r"^(\d{9})$").unwrap();
}

pub fn part2(input: String) -> String {
    let passports: Vec<&str> = input.split("\n\n").collect();
    let mut count = 0;

    let required_fields: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    for passport in passports {
        let mut temp: HashMap<String, String> = HashMap::new();

        let fields: Vec<(&str, &str)> = passport
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .map(|s| {
                let pair: Vec<&str> = s.split(":").collect();
                (pair[0], pair[1])
            })
            .collect();
        for (key, val) in fields {
            temp.insert(key.to_owned(), val.to_owned());
        }
        let mut valid = true;
        for rf in required_fields.iter() {
            if !temp.contains_key(&rf.to_string()) {
                valid = false;
                break;
            }
            let val = temp.get(&rf.to_string()).unwrap().to_string();
            valid = match *rf {
                // byr (Birth Year) - four digits; at least 1920 and at most 2002.
                "byr" => {
                    match val.parse::<i32>() {
                        Ok(n) => 1920 <= n && n <= 2002,
                        Err(_) => false,
                    }
                },
                // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
                "iyr" => {
                    match val.parse::<i32>() {
                        Ok(n) => 2010 <= n && n <= 2020,
                        Err(_) => false,
                    }
                },
                // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
                "eyr" => {
                    match val.parse::<i32>() {
                        Ok(n) => 2020 <= n && n <= 2030,
                        Err(_) => false,
                    }
                },
                // hgt (Height) - a number followed by either cm or in:
                //     If cm, the number must be at least 150 and at most 193.
                //     If in, the number must be at least 59 and at most 76.
                "hgt" => {
                    match HGT.captures(&val) {
                        Some(result) => {
                            let unit = result.name("unit").map_or("", |m| m.as_str());
                            let num = result
                                .name("num")
                                .map_or("", |m| m.as_str())
                                .parse::<i32>()
                                .unwrap_or(0);
                            match unit {
                                "cm" => 150 <= num && num <= 193,
                                "in" => 59 <= num && num <= 76,
                                _ => false
                            }
                        },
                        None => false,
                    }
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
    count.to_string()
}
