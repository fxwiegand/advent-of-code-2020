use itertools::Itertools;
use std::str::FromStr;

pub(crate) fn solve_day9() -> u64 {
    let input_file = include_str!("../resources/day9.txt");
    let numbers = input_file
        .lines()
        .map(|n| u64::from_str(n).unwrap())
        .collect_vec();

    let mut result = 0;

    'outer: for (index, n) in numbers.iter().skip(25).enumerate() {
        let i = index + 25;
        for j in (i - 25)..i {
            for k in (i - 25)..i {
                if &((numbers[j] + numbers[k]) as u64) == n {
                    continue 'outer;
                }
            }
        }
        result = *n;
        break;
    }

    result
}

pub(crate) fn solve_day9_part2() -> u64 {
    let input_file = include_str!("../resources/day9.txt");
    let numbers = input_file
        .lines()
        .map(|n| u64::from_str(n).unwrap())
        .collect_vec();

    let mut result = Vec::new();
    let number = solve_day9();
    'outer: for i in 0..numbers.len() {
        for k in (i + 2)..numbers.len() {
            if numbers.iter().skip(i).take(k - i).sum::<u64>() == number {
                result = numbers.iter().skip(i).take(k - i).collect();
                break 'outer;
            }
        }
    }
    *result.iter().min().unwrap() + *result.iter().max().unwrap()
}
