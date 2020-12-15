use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

pub(crate) fn solve_day15() -> u64 {
    solve_day15_general(2020)
}

pub(crate) fn solve_day15_part2() -> u64 {
    solve_day15_general(30000000)
}

fn solve_day15_general(stop: usize) -> u64 {
    let input_file = include_str!("day15.txt");
    let input = input_file.lines().collect_vec()[0]
        .split(',')
        .map(|s| u64::from_str(s).unwrap())
        .collect_vec();

    let mut index = input.len().clone();
    let mut map = HashMap::new();
    for (p, i) in input.into_iter().enumerate() {
        let r = map.entry(i).or_insert_with(Vec::new);
        r.push(p);
    }
    let mut next = 0;
    loop {
        if index == stop - 1 {
            break;
        }
        let current = next;
        let r = map.entry(current).or_insert_with(Vec::new);
        r.push(index);
        match map.get(&current) {
            Some(v) => {
                if v.len() > 1 {
                    let last_two = v.iter().rev().take(2).rev().collect_vec();
                    next = (last_two[1] - last_two[0]) as u64;
                } else {
                    next = 0;
                }
            }
            None => {
                next = 0;
            }
        }
        index += 1;
    }
    next
}
