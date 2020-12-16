use itertools::Itertools;
use std::str::FromStr;

pub(crate) fn solve_day10() -> u64 {
    let input_file = include_str!("../resources/day10.txt");
    let numbers = input_file
        .lines()
        .map(|n| u64::from_str(n).unwrap())
        .sorted()
        .collect_vec();

    let mut last = 0;
    let mut one_joltages = 0;
    let mut three_joltages = 0;

    for n in numbers {
        match (n - last == 1, n - last == 3) {
            (true, false) => one_joltages += 1,
            (false, true) => three_joltages += 1,
            (_, _) => {}
        }
        last = n;
    }
    three_joltages += 1;
    one_joltages * three_joltages
}

pub(crate) fn solve_day10_part2() -> u64 {
    let input_file = include_str!("../resources/day10.txt");
    let numbers = input_file
        .lines()
        .map(|n| u64::from_str(n).unwrap())
        .sorted()
        .collect_vec();

    let mut last = 0;
    let mut joltages = Vec::new();

    for n in numbers {
        joltages.push(n - last);
        last = n;
    }
    joltages.push(3);

    let mut succesive_one_joltages = Vec::new();

    let last = joltages.len() - 1;
    let mut streak = 0;
    for (i, jolt) in joltages.into_iter().enumerate() {
        if jolt == 1 {
            streak += 1;
        } else {
            succesive_one_joltages.push(streak);
            streak = 0;
        }
        if i == last {
            succesive_one_joltages.push(streak);
        }
    }

    let mut result = 1;
    for s in succesive_one_joltages {
        match s {
            2 => result = result * 2,
            3 => result = result * 4,
            4 => result = result * 7,
            5 => result = result * 13,
            _ => {}
        };
    }

    result
}
