use itertools::partition;
use std::str::FromStr;

pub(crate) fn solve_day1() -> u32 {
    let mut input = Vec::new();
    let input_file = include_str!("day1.txt");
    for s in input_file.split_whitespace() {
        input.push(u32::from_str(s).unwrap());
    }

    let split_index = partition(&mut input, |elt| *elt >= 1010);
    let (high, low) = input.split_at(split_index);

    let mut result1 = 0;
    let mut result2 = 0;
    for h in high {
        for l in low {
            if h + l == 2020 {
                result1 = *h;
                result2 = *l;
                break;
            }
        }
    }
    result2 * result1
}

pub(crate) fn solve_day1_part2() -> u32 {
    let mut input = Vec::new();
    let input_file = include_str!("day1.txt");
    for s in input_file.split_whitespace() {
        input.push(u32::from_str(s).unwrap());
    }

    let mut result1 = 0;
    let mut result2 = 0;
    let mut result3 = 0;
    for h in &input {
        for l in &input {
            for n in &input {
                if h + l + n == 2020 {
                    result1 = *h;
                    result2 = *l;
                    result3 = *n;
                    break;
                }
            }
        }
    }
    result2 * result1 * result3
}
