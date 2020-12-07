use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::str::FromStr;

pub(crate) fn solve_day7() -> u32 {
    let input_file = include_str!("day7.txt");
    let restrictions = input_file
        .lines()
        .collect_vec()
        .into_iter()
        .map(|s| s.trim_end_matches('.'))
        .collect_vec();

    let mut bag_map = HashMap::new();
    for r in restrictions {
        let tmp = r.split(" contain ").collect_vec();
        let outer_bag = tmp[0].trim_end_matches('s');
        let contained_bags = tmp[1]
            .split(',')
            .collect_vec()
            .iter()
            .map(|s| s.trim_start())
            .collect_vec();
        let entry = bag_map.entry(outer_bag.to_owned()).or_insert_with(Vec::new);
        for bag in contained_bags {
            if !bag.starts_with("no") {
                let quantity =
                    u32::from_str(bag.clone().split_whitespace().collect_vec()[0]).unwrap();
                let bag_without_quantity = bag.split_at(2).1.trim_end_matches('s').to_owned();
                entry.push(bag_without_quantity);
            }
        }
    }

    for (_, mut inner_bags) in bag_map.clone() {
        inner_bags.sort();
        inner_bags.dedup();
    }

    let mut count_outer_bags = 0;

    for (outer_bag, _) in bag_map.clone() {
        dbg!(outer_bag.clone());
        if check_bag(outer_bag, bag_map.clone()) {
            count_outer_bags += 1;
        }
    }

    count_outer_bags - 1
}

fn check_bag(bag_to_check: String, bag_map: HashMap<String, Vec<String>>) -> bool {
    if bag_to_check == "shiny gold bag" {
        return true;
    } else {
        if let Some(other_bags_to_check) = bag_map.get(&bag_to_check) {
            for b in other_bags_to_check {
                if check_bag(b.clone(), bag_map.clone()) {
                    return true;
                }
            }
        }
    }
    return false;
}

pub(crate) fn solve_day7_part2() -> u64 {
    let input_file = include_str!("day7.txt");
    let restrictions = input_file
        .lines()
        .collect_vec()
        .into_iter()
        .map(|s| s.trim_end_matches('.'))
        .collect_vec();

    let mut bag_map = HashMap::new();
    for r in restrictions {
        let tmp = r.split(" contain ").collect_vec();
        let outer_bag = tmp[0].trim_end_matches('s');
        let contained_bags = tmp[1]
            .split(',')
            .collect_vec()
            .iter()
            .map(|s| s.trim_start())
            .collect_vec();
        let entry = bag_map.entry(outer_bag.to_owned()).or_insert_with(Vec::new);
        for bag in contained_bags {
            if !bag.starts_with("no") {
                let quantity =
                    u32::from_str(bag.clone().split_whitespace().collect_vec()[0]).unwrap();
                let bag_without_quantity = bag.split_at(2).1.trim_end_matches('s').to_owned();
                entry.push((bag_without_quantity, quantity));
            }
        }
    }

    count_bags_inside("shiny gold bag".parse().unwrap(), bag_map.clone()) - 1
}

fn count_bags_inside(bag_to_check: String, bag_map: HashMap<String, Vec<(String, u32)>>) -> u64 {
    let mut sum = 1;
    if let Some(other_bags_to_check) = bag_map.get(&bag_to_check) {
        if other_bags_to_check.is_empty() {
            return 1;
        } else {
            for (b, quant) in other_bags_to_check {
                sum = sum + (*quant as u64 * count_bags_inside(b.clone(), bag_map.clone()));
            }
        }
    }
    return sum;
}
