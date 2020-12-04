use itertools::Itertools;
use std::str::FromStr;
use regex::Regex;

pub(crate) fn solve_day4() -> u32 {
    let input_file = include_str!("day4.txt");
    let fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let optionals = vec!["cid"];
    let rows = input_file.split("\n\n").collect_vec();
    let mut count_valid = 0;
    for r in rows {
        let mut valid = true;
        for f in &fields {
            if !r.contains(f) {
                valid = false;
            }
        }
        if valid {
            count_valid += 1;
        }
    }
    count_valid
}

pub(crate) fn solve_day4_part2() -> u32 {
    let input_file = include_str!("day4.txt");
    let fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let optionals = vec!["cid"];
    let rows = input_file.split("\n\n").collect_vec();
    let mut count_valid = 0;
    for r in rows {
        let mut valid = true;
        for f in &fields {
            if !r.contains(f) {
                valid = false;
            }
        }
        let pairs = r.split_whitespace().collect_vec();
        for pair in pairs {
            let tmp = pair.split(':').collect_vec();
            let field = tmp[0];
            let val = tmp[1];
            match field {
                "byr" => {
                    if !(val.len() == 4) {
                        valid = false;
                    }
                    let num = u32::from_str(val).unwrap();
                    if !(num >= 1920 && num <= 2002) {
                        valid = false;
                    }
                },
                "iyr" => {
                    if !(val.len() == 4) {
                        valid = false;
                    }
                    let num = u32::from_str(val).unwrap();
                    if !(num >= 2010 && num <= 2020) {
                        valid = false;
                    }
                },
                "eyr" => {
                    if !(val.len() == 4) {
                        valid = false;
                    }
                    let num = u32::from_str(val).unwrap();
                    if !(num >= 2020 && num <= 2030) {
                        valid = false;
                    }
                },
                "hgt" => {
                    if val.ends_with("cm") {
                        let h = u32::from_str(val.trim_end_matches("cm")).unwrap();
                        if !(h >= 150 && h <= 193) {
                            valid = false;
                        }
                    } else if val.ends_with("in") {
                        let h = u32::from_str(val.trim_end_matches("in")).unwrap();
                        if !(h >= 59 && h <= 76) {
                            valid = false;
                        }
                    } else {
                        valid = false;
                    }
                },
                "hcl" => {
                    let re = Regex::new(r"^#(?:[0-9a-fA-F]{3}){1,2}$").unwrap();
                    if !re.is_match(val) {
                        valid = false;
                    }
                },
                "ecl" => {
                    let col_options = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                    if !col_options.contains(&val) {
                        valid = false;
                    }
                },
                "pid" => {
                    let re = Regex::new(r"^\d{9}$").unwrap();
                    if !re.is_match(val) {
                        valid = false;
                    }
                }
                _ => {}
            }
        }
        if valid {
            count_valid += 1;
        }
    }
    count_valid
}