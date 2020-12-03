use itertools::Itertools;

pub(crate) fn solve_day3() -> u32 {
    let input_file = include_str!("day3.txt");
    let rows = input_file
        .lines()
        .map(|s| s.chars().collect_vec())
        .collect_vec();
    let row_len = rows[0].len();
    let mut down = 0;
    let mut right = 0;
    let mut trees = 0;
    while down < rows.len() {
        if rows[down][right % row_len] == '#' {
            trees += 1;
        }
        down += 1;
        right += 3;
    }
    trees
}

pub(crate) fn solve_day3_part2() -> u64 {
    let input_file = include_str!("day3.txt");
    let rows = input_file
        .lines()
        .map(|s| s.chars().collect_vec())
        .collect_vec();

    let mut result = traverse(rows.clone(), 1, 1);
    result = result * traverse(rows.clone(), 3, 1);
    result = result * traverse(rows.clone(), 5, 1);
    result = result * traverse(rows.clone(), 7, 1);
    result = result * traverse(rows.clone(), 1, 2);
    result
}

fn traverse(rows: Vec<Vec<char>>, right_step: usize, down_step: usize) -> u64 {
    let row_len = rows[0].len();
    let mut down = 0;
    let mut right = 0;
    let mut trees = 0;
    while down < rows.len() {
        if rows[down][right % row_len] == '#' {
            trees += 1;
        }
        down += down_step;
        right += right_step;
    }
    trees
}
