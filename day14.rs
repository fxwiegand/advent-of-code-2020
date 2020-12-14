use itertools::Itertools;
use std::collections::{BTreeMap, HashMap};
use std::str::FromStr;

pub(crate) fn solve_day14() -> u64 {
    let input_file = include_str!("day14.txt");
    let mut input = input_file.lines().collect_vec();

    let mut masks = Vec::new();
    let mut mask = "";
    let mut tmp_mask = "";
    let mut tmp_masks = Vec::new();
    for line in input {
        let tmp = line.split_whitespace().collect_vec();
        match tmp[0] {
            "mask" => {
                masks.push((mask, tmp_masks));
                tmp_masks = Vec::new();
                mask = tmp[2];
            }
            _ => {
                let mem_adress =
                    u64::from_str(tmp[0].trim_end_matches(']').trim_start_matches("mem[")).unwrap();
                tmp_masks.push(((mem_adress, u64::from_str(tmp[2]).unwrap())));
            }
        };
    }
    masks.push((mask, tmp_masks));

    dbg!(masks.clone());
    let mut memory = vec![0_u64; 100000];
    for (mask, numbers) in masks {
        for (me, num) in numbers {
            memory[me as usize] = bit_or(mask, num);
        }
    }

    memory.iter().sum()
}

fn bit_or(mask: &str, i: u64) -> u64 {
    let mut p = i;
    let mut num_str = String::new();
    while p > 0 {
        if p % 2 == 0 {
            num_str.push('0');
        } else {
            num_str.push('1')
        }
        p /= 2;
    }
    if mask.len() > num_str.len() {
        let n = mask.len() - num_str.len();
        for _ in 0..n {
            num_str.push('0');
        }
    }
    num_str = num_str.chars().rev().collect();
    let mut result = 0;
    for (e, (bit_mask, bit_int)) in mask.chars().rev().zip_eq(num_str.chars().rev()).enumerate() {
        match (bit_mask, bit_int) {
            ('X', a) => {
                if a == '1' {
                    result += (2u64.pow(e as u32))
                }
            }
            ('1', _) => result += (2u64.pow(e as u32)),
            ('0', _) => {}
            _ => unreachable!(),
        }
    }
    result
}

pub(crate) fn solve_day14_part2() -> u64 {
    let input_file = include_str!("day14.txt");
    let mut input = input_file.lines().collect_vec();

    let mut masks = Vec::new();
    let mut mask = "";
    let mut tmp_mask = "";
    let mut tmp_masks = Vec::new();
    for line in input {
        let tmp = line.split_whitespace().collect_vec();
        match tmp[0] {
            "mask" => {
                masks.push((mask, tmp_masks));
                tmp_masks = Vec::new();
                mask = tmp[2];
            }
            _ => {
                let mem_adress =
                    u64::from_str(tmp[0].trim_end_matches(']').trim_start_matches("mem[")).unwrap();
                tmp_masks.push(((mem_adress, u64::from_str(tmp[2]).unwrap())));
            }
        };
    }
    masks.push((mask, tmp_masks));

    let mut memory = HashMap::new();
    for (mask, numbers) in masks {
        for (me, num) in numbers {
            for res in bit_or2(mask, me) {
                memory.insert(res, num);
            }
        }
    }

    memory.values().sum()
}

fn bit_or2(mask: &str, i: u64) -> Vec<u64> {
    let mut p = i;
    let mut num_str = String::new();
    while p > 0 {
        if p % 2 == 0 {
            num_str.push('0');
        } else {
            num_str.push('1')
        }
        p /= 2;
    }
    if mask.len() > num_str.len() {
        let n = mask.len() - num_str.len();
        for _ in 0..n {
            num_str.push('0');
        }
    }
    num_str = num_str.chars().rev().collect();
    let xses = mask.chars().filter(|c| *c == 'X').collect_vec().len();
    let mut result = Vec::new();
    let mut s_result = 0;
    result.push(0);
    for (e, (bit_mask, bit_int)) in mask.chars().rev().zip_eq(num_str.chars().rev()).enumerate() {
        match (bit_mask, bit_int) {
            ('X', a) => {
                for el in result.clone() {
                    let new_el = el + (2u64.pow(e as u32));
                    result.push(new_el);
                }
            }
            ('1', _) => {
                s_result += (2u64.pow(e as u32));
            }
            ('0', a) => {
                if a == '1' {
                    s_result += (2u64.pow(e as u32));
                }
            }
            _ => unreachable!(),
        }
    }
    let mut final_res = Vec::new();
    for mut elem in &result {
        final_res.push(elem + s_result);
    }
    final_res
}
