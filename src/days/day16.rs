use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

pub(crate) fn solve_day16() -> u32 {
    let input_file = include_str!("../resources/day16.txt");
    let temp = input_file.split("\n\n").collect_vec();

    let min_max = temp[0]
        .lines()
        .collect_vec()
        .iter()
        .map(|line| {
            let tmp = line.split(" or ").collect_vec();
            let last_range = tmp[1]
                .split('-')
                .map(|i| u32::from_str(i).unwrap())
                .collect_vec();
            let first_range_tmp = tmp[0].split_whitespace().last().unwrap();
            let first_range = first_range_tmp
                .split('-')
                .map(|i| u32::from_str(i).unwrap())
                .collect_vec();
            (first_range[0], first_range[1], last_range[0], last_range[1])
        })
        .collect_vec();

    let nearby_tickets = temp[2]
        .lines()
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|i| u32::from_str(i).unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut error_rate = 0;
    for ticket in nearby_tickets {
        for value in ticket {
            let mut valid = false;
            for (min1, max1, min2, max2) in &min_max {
                match (
                    value >= *min1 && value <= *max1,
                    value >= *min2 && value <= *max2,
                ) {
                    (true, _) | (_, true) => {
                        valid = true;
                        break;
                    }
                    (_, _) => {}
                }
            }
            if !valid {
                error_rate += value;
            }
        }
    }
    error_rate
}

pub(crate) fn solve_day16_part2() -> u64 {
    let input_file = include_str!("../resources/day16.txt");
    let temp = input_file.split("\n\n").collect_vec();

    let my_ticket = temp[1]
        .lines()
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|i| u32::from_str(i).unwrap())
                .collect_vec()
        })
        .collect_vec()[0]
        .clone();

    let min_max = temp[0]
        .lines()
        .collect_vec()
        .iter()
        .map(|line| {
            let tmp = line.split(" or ").collect_vec();
            let last_range = tmp[1]
                .split('-')
                .map(|i| u32::from_str(i).unwrap())
                .collect_vec();
            let first_range_tmp = tmp[0].split_whitespace().last().unwrap();
            let first_range = first_range_tmp
                .split('-')
                .map(|i| u32::from_str(i).unwrap())
                .collect_vec();
            (first_range[0], first_range[1], last_range[0], last_range[1])
        })
        .collect_vec();

    let row_names = temp[0]
        .lines()
        .collect_vec()
        .iter()
        .map(|line| line.split(":").collect_vec()[0])
        .collect_vec();

    let nearby_tickets = temp[2]
        .lines()
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|i| u32::from_str(i).unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut filtered_tickets = Vec::new();
    for ticket in nearby_tickets {
        let mut ticket_valid = true;
        for value in &ticket {
            let mut valid = false;
            for (min1, max1, min2, max2) in &min_max {
                match (
                    value >= min1 && value <= max1,
                    value >= min2 && value <= max2,
                ) {
                    (true, _) | (_, true) => {
                        valid = true;
                        break;
                    }
                    (_, _) => {}
                }
            }
            if !valid {
                ticket_valid = false;
                break;
            }
        }
        if ticket_valid {
            filtered_tickets.push(ticket);
        }
    }

    let mut possible_candidates = HashMap::new();
    for ticket_attr in 0..my_ticket.len() {
        for column in 0..my_ticket.len() {
            let mut valid = true;
            for ticket in &filtered_tickets {
                let value = ticket[column];
                let (min1, max1, min2, max2) = min_max[ticket_attr];
                match (
                    value >= min1 && value <= max1,
                    value >= min2 && value <= max2,
                ) {
                    (false, false) => {
                        valid = false;
                        break;
                    }
                    (_, _) => {}
                }
            }
            if valid {
                let record = possible_candidates.entry(column).or_insert_with(Vec::new);
                record.push(row_names[ticket_attr]);
            }
        }
    }

    let mut go = true;
    let mut already_rem = Vec::new();
    let mut map = possible_candidates.clone();
    while go {
        (go, map, already_rem) = reduce_map(map.clone(), already_rem.clone());
    }

    let mut result = 1;
    for (k, v) in map {
        if v[0].starts_with("departure") {
            result *= my_ticket[k] as u64;
        }
    }

    result
}

fn reduce_map(
    possible_candidates: HashMap<usize, Vec<&str>>,
    already_rem: Vec<String>,
) -> (bool, HashMap<usize, Vec<&str>>, Vec<String>) {
    let mut remove = "";
    let mut except = 0;
    let mut ar = already_rem.clone();
    for (col, pos_vals) in &possible_candidates {
        if pos_vals.len() == 1 {
            if !already_rem.contains(&pos_vals[0].to_string()) {
                remove = pos_vals[0];
                ar.push(pos_vals[0].to_string());
                except = *col;
                break;
            }
        }
    }
    if remove.is_empty() {
        return (false, possible_candidates.clone(), ar);
    }
    let mut another_pos_cand = possible_candidates.clone();
    for (col, pos_vals) in &possible_candidates {
        if col != &except {
            let x = another_pos_cand.get_mut(col).unwrap();
            *x = pos_vals
                .iter()
                .filter(|s| s != &&remove)
                .map(|s| s.to_owned())
                .collect_vec();
        }
    }
    (true, another_pos_cand, ar)
}
