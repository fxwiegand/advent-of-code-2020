use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

pub(crate) fn solve_day8() -> i64 {
    let input_file = include_str!("../resources/day8.txt");
    let operations = input_file
        .lines()
        .collect_vec()
        .into_iter()
        .map(|s| {
            (
                s.split_whitespace().collect_vec()[0],
                i64::from_str(s.split_whitespace().collect_vec()[1]).unwrap(),
            )
        })
        .collect_vec();

    let mut op: i64 = 0;
    let mut acc = 0;
    let mut executed_lines = HashSet::new();
    while executed_lines.insert(op) {
        let n = operations[op as usize].1;
        match operations[op as usize].0 {
            "jmp" => op += n,
            "acc" => {
                acc += n;
                op += 1;
            }
            "nop" => op += 1,
            _ => unreachable!(),
        }
        dbg!(op);
    }

    acc
}

pub(crate) fn solve_day8_part2() -> i64 {
    let input_file = include_str!("../resources/day8.txt");
    let mut operations = input_file
        .lines()
        .collect_vec()
        .into_iter()
        .map(|s| {
            (
                s.split_whitespace().collect_vec()[0],
                i64::from_str(s.split_whitespace().collect_vec()[1]).unwrap(),
            )
        })
        .collect_vec();

    let mut result = 0;
    'outer: for (i, _) in operations.clone().iter().enumerate() {
        let o;
        if operations[i].0 == "jmp" {
            operations[i].0 = "nop";
            o = true;
        } else if operations[i].0 == "nop" {
            operations[i].0 = "jmp";
            o = false;
        } else {
            continue;
        }
        dbg!(operations.clone());
        let mut op: i64 = 0;
        let mut acc = 0;
        let mut executed_lines = HashSet::new();
        while executed_lines.insert(op) {
            if op < operations.len() as i64 {
                let n = operations[op as usize].1;
                match operations[op as usize].0 {
                    "jmp" => op += n,
                    "acc" => {
                        acc += n;
                        op += 1;
                    }
                    "nop" => op += 1,
                    _ => unreachable!(),
                };
            } else {
                dbg!("Terminated");
                result = acc;
                break 'outer;
            }
        }
        if o {
            operations[i].0 = "jmp"
        } else {
            operations[i].0 = "nop"
        }
    }
    result
}
