use itertools::Itertools;
use std::collections::HashSet;

pub(crate) fn solve_day6() -> u32 {
    let input_file = include_str!("../resources/day6.txt");
    input_file
        .split("\n\n")
        .collect_vec()
        .iter()
        .map(|s| {
            s.chars()
                .unique()
                .filter(|c| *c != '\n')
                .collect_vec()
                .len() as u32
        })
        .sum()
}

pub(crate) fn solve_day6_part2() -> u32 {
    let input_file = include_str!("../resources/day6.txt");
    input_file
        .split("\n\n")
        .collect_vec()
        .iter()
        .map(|s| {
            let answers = s
                .lines()
                .collect_vec()
                .iter()
                .map(|s| s.chars().filter(|c| *c != '\n').collect::<HashSet<char>>())
                .collect_vec();
            let mut intersection = answers[0].clone();
            for person_answers in answers.iter().skip(1) {
                intersection = intersection
                    .intersection(person_answers)
                    .map(|s| s.clone())
                    .collect();
            }
            intersection.len() as u32
        })
        .sum()
}
