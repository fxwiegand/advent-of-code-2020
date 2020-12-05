use itertools::Itertools;
use std::cmp::max;

pub(crate) fn solve_day5() -> u32 {
    let input_file = include_str!("day5.txt");
    let rows = input_file
        .lines()
        .map(|s| s.chars().collect_vec())
        .collect_vec();

    let mut seat_ids = Vec::new();
    for seat in rows {
        let mut bit_string = Vec::new();
        for c in seat.iter().take(7) {
            match c {
                'F' => bit_string.push(0),
                'B' => bit_string.push(1),
                _ => unreachable!(),
            }
        }
        let seat_row = convert(bit_string.as_slice()) as u32;
        let mut bit_string2 = Vec::new();
        for _ in 0..5 {
            bit_string2.push(0);
        }
        for c in seat.iter().skip(7).take(3) {
            match c {
                'L' => bit_string2.push(0),
                'R' => bit_string2.push(1),
                _ => unreachable!(),
            }
        }
        let seat_side_row = convert(bit_string2.as_slice()) as u32;
        seat_ids.push(seat_row * 8 + seat_side_row);
    }
    seat_ids.iter().max().unwrap().clone()
}

pub(crate) fn solve_day5_part2() -> u32 {
    let input_file = include_str!("day5.txt");
    let rows = input_file
        .lines()
        .map(|s| s.chars().collect_vec())
        .collect_vec();

    let mut seat_ids = Vec::new();
    for seat in rows {
        let mut bit_string = Vec::new();
        for c in seat.iter().take(7) {
            match c {
                'F' => bit_string.push(0),
                'B' => bit_string.push(1),
                _ => unreachable!(),
            }
        }
        let seat_row = convert(bit_string.as_slice()) as u32;
        let mut bit_string2 = Vec::new();
        for _ in 0..5 {
            bit_string2.push(0);
        }
        for c in seat.iter().skip(7).take(3) {
            match c {
                'L' => bit_string2.push(0),
                'R' => bit_string2.push(1),
                _ => unreachable!(),
            }
        }
        let seat_side_row = convert(bit_string2.as_slice()) as u32;
        seat_ids.push(seat_row * 8 + seat_side_row);
    }
    seat_ids.sort();
    let mut my_id = 0;
    let mut start = seat_ids[0];
    for id in seat_ids.iter().skip(1) {
        if !(start + 1 == *id) {
            my_id = start + 1;
            break;
        } else {
            start = *id;
        }
    }
    my_id
}

fn convert(bits: &[u8]) -> u8 {
    bits.iter().fold(0, |result, &bit| (result << 1) ^ bit)
}
