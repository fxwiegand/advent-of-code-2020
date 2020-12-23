use itertools::Itertools;
use std::str::FromStr;

pub(crate) fn solve_day23() -> String {
    let input_file = include_str!("../resources/day23.txt");
    let mut cups = input_file
        .chars()
        .map(|c| u32::from_str(&c.to_string()).unwrap())
        .collect_vec();

    let mut current_cup_index = 0;

    for _ in 0..100 {
        let current_cup = cups[current_cup_index];

        let pick_ups = if current_cup_index > 5 {
            [
                &cups[(current_cup_index + 1)..=8],
                &cups[0..((current_cup_index + 4) % 9)],
            ]
            .concat()
        } else {
            cups[(current_cup_index + 1)..current_cup_index + 4].to_vec()
        };

        let mut destination_cup = 0;
        for mi in 1..5 {
            if cups[current_cup_index] as i32 - mi as i32 > 0 {
                if !pick_ups.contains(&(cups[current_cup_index] - mi)) {
                    destination_cup = cups[current_cup_index] - mi;
                    break;
                } else {
                    continue;
                }
            } else {
                match cups[current_cup_index] as i32 - mi as i32 {
                    0 => {
                        if !pick_ups.contains(&(9 as u32)) {
                            destination_cup = 9;
                            break;
                        } else {
                            continue;
                        }
                    }
                    -1 => {
                        if !pick_ups.contains(&(8 as u32)) {
                            destination_cup = 8;
                            break;
                        } else {
                            continue;
                        }
                    }
                    -2 => {
                        if !pick_ups.contains(&(7 as u32)) {
                            destination_cup = 7;
                            break;
                        } else {
                            continue;
                        }
                    }
                    -3 => {
                        if !pick_ups.contains(&(6 as u32)) {
                            destination_cup = 6;
                            break;
                        } else {
                            continue;
                        }
                    }
                    _ => {
                        unreachable!()
                    }
                }
            }
        }
        assert_ne!(destination_cup, 0);
        let mut new_cups = cups
            .iter()
            .filter(|a| !pick_ups.contains(a))
            .map(|x| *x)
            .collect_vec();
        let next_cup = new_cups[(new_cups.iter().position(|&r| r == current_cup).unwrap() + 1) % 6];
        let destination_cup_index = new_cups.iter().position(|&r| r == destination_cup).unwrap();
        for p in pick_ups.into_iter().rev() {
            new_cups.insert(destination_cup_index + 1, p);
        }
        let next_cup_index = new_cups.iter().position(|&r| r == next_cup).unwrap();
        current_cup_index = next_cup_index;
        cups = new_cups;
    }

    let one_index = cups.iter().position(|&r| r == 1).unwrap();
    let result = [&cups[one_index + 1..=8], &cups[0..one_index]].concat();
    let mut res_str = String::new();
    for r in result {
        res_str.push_str(&r.to_string());
    }
    res_str
}
