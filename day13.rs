use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

pub(crate) fn solve_day13() -> i32 {
    let input_file = include_str!("day13.txt");
    let dep_time = input_file
        .lines()
        .take(1)
        .map(|s| i32::from_str(s).unwrap())
        .collect_vec()[0];

    let bus_ids = input_file.lines().skip(1).collect_vec()[0]
        .split(',')
        .filter(|s| s != &"x")
        .map(|s| i32::from_str(s).unwrap())
        .collect_vec();

    let mut waiting_times_and_ids = Vec::new();
    for id in &bus_ids {
        waiting_times_and_ids.push((((-dep_time % id) + id) % id, id));
    }

    let (waiting_time, bus) = waiting_times_and_ids
        .iter()
        .min_by(|(x, _), (y, _)| x.cmp(&y))
        .unwrap();
    *bus * waiting_time
}

pub(crate) fn solve_day13_part2() -> u128 {
    let input_file = include_str!("day13.txt");
    let bus_ids = input_file.lines().skip(1).collect_vec()[0]
        .split(',')
        .map(|s| {
            if s != "x" {
                u128::from_str(s).unwrap()
            } else {
                0
            }
        })
        .collect_vec()
        .into_iter()
        .enumerate()
        .filter(|(_, x)| x != &0)
        .collect_vec();

    let mut t = 0;
    let mut step = bus_ids[0].1;
    let mut already_found = HashSet::new();
    already_found.insert(step);
    'outer: loop {
        t += step;
        for (i, bid) in &bus_ids {
            if !((t + *i as u128) % bid == 0) {
                continue 'outer;
            } else {
                if !already_found.contains(bid) {
                    step = step * bid;
                    already_found.insert(*bid);
                }
            }
        }
        break 'outer;
    }

    t
}
