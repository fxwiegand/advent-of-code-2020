use itertools::izip;
use itertools::Itertools;
use std::str::FromStr;

pub(crate) fn solve_day2() -> u32 {
    let input_file = include_str!("../resources/day2.txt");
    let mut ranges = Vec::new();
    let mut chars = Vec::new();
    let mut pwds = Vec::new();
    for s in input_file.lines() {
        let tmp = s.split_whitespace().collect_vec();
        let tmp_range = tmp[0];
        let tmp_char = tmp[1];
        let pwd = tmp[2];

        let range = tmp_range
            .split('-')
            .map(|s| u32::from_str(s).unwrap())
            .collect_vec();
        ranges.push((range[0], range[1]));
        let char = tmp_char.chars().collect_vec()[0];
        chars.push(char);
        pwds.push(pwd.to_owned());
    }

    let mut count_valid = 0;
    for (password, c, rng) in izip!(pwds, chars, ranges) {
        let valid = check_pwd(rng.0, rng.1, c, password);
        if valid {
            count_valid += 1;
        }
    }
    count_valid
}

fn check_pwd(lower_bound: u32, upper_bound: u32, c: char, password: String) -> bool {
    let mut count = 0;
    for pwd_char in password.chars() {
        if pwd_char == c {
            count += 1;
        }
    }
    count >= lower_bound && count <= upper_bound
}

pub(crate) fn solve_day2_part2() -> u32 {
    let input_file = include_str!("../resources/day2.txt");
    let mut ranges = Vec::new();
    let mut chars = Vec::new();
    let mut pwds = Vec::new();
    for s in input_file.lines() {
        let tmp = s.split_whitespace().collect_vec();
        let tmp_range = tmp[0];
        let tmp_char = tmp[1];
        let pwd = tmp[2];

        let range = tmp_range
            .split('-')
            .map(|s| u32::from_str(s).unwrap())
            .collect_vec();
        ranges.push((range[0], range[1]));
        let char = tmp_char.chars().collect_vec()[0];
        chars.push(char);
        pwds.push(pwd.to_owned());
    }

    let mut count_valid = 0;
    for (password, c, rng) in izip!(pwds, chars, ranges) {
        let valid = check_pwd_part2(rng.0, rng.1, c, password);
        if valid {
            count_valid += 1;
        }
    }
    count_valid
}

fn check_pwd_part2(lower_bound: u32, upper_bound: u32, c: char, password: String) -> bool {
    let pwd_chars = password.chars().collect_vec();
    match (
        pwd_chars[lower_bound as usize - 1] == c,
        pwd_chars[upper_bound as usize - 1] == c,
    ) {
        (true, false) | (false, true) => true,
        _ => false,
    }
}
