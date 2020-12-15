use itertools::Itertools;

pub(crate) fn solve_day11() -> u32 {
    let input_file = include_str!("day11.txt");
    let mut seats = input_file
        .lines()
        .collect_vec()
        .into_iter()
        .map(|s| s.chars().collect_vec())
        .collect_vec();

    loop {
        let new_seats = update_seats(&seats);
        if seats == new_seats {
            break;
        }
        seats = new_seats;
    }

    count_seats(&seats)
}

fn count_seats(seats: &Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    for row in seats {
        for seat in row {
            if seat == &'#' {
                count += 1;
            }
        }
    }
    count
}

fn update_seats(seats: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_seats = seats.clone();
    for (i, row) in seats.iter().enumerate() {
        for (k, seat) in row.iter().enumerate() {
            match seat {
                &'#' => {
                    let mut tmp_seat_vec = Vec::new();
                    if i as i32 - 1 >= 0 {
                        tmp_seat_vec.push(seats[i - 1][k]);
                        if (k as i32 - 1) >= 0 {
                            tmp_seat_vec.push(seats[i - 1][k - 1]);
                        }
                        if (k + 1) < row.len() {
                            tmp_seat_vec.push(seats[i - 1][k + 1]);
                        }
                    }
                    if i + 1 < seats.len() {
                        tmp_seat_vec.push(seats[i + 1][k]);
                        if (k as i32 - 1) >= 0 {
                            tmp_seat_vec.push(seats[i + 1][k - 1]);
                        }
                        if (k + 1) < row.len() {
                            tmp_seat_vec.push(seats[i + 1][k + 1]);
                        }
                    }
                    if (k as i32 - 1) >= 0 {
                        tmp_seat_vec.push(seats[i][k - 1]);
                    }
                    if (k + 1) < row.len() {
                        tmp_seat_vec.push(seats[i][k + 1]);
                    }
                    let mut count = 0;
                    for s in tmp_seat_vec {
                        if s == '#' {
                            count += 1;
                        }
                    }
                    if count >= 4 {
                        new_seats[i][k] = 'L'
                    }
                }
                &'L' => {
                    let mut tmp_seat_vec = Vec::new();
                    if i as i32 - 1 >= 0 {
                        tmp_seat_vec.push(seats[i - 1][k]);
                        if (k as i32 - 1) >= 0 {
                            tmp_seat_vec.push(seats[i - 1][k - 1]);
                        }
                        if (k + 1) < row.len() {
                            tmp_seat_vec.push(seats[i - 1][k + 1]);
                        }
                    }
                    if i + 1 < seats.len() {
                        tmp_seat_vec.push(seats[i + 1][k]);
                        if (k as i32 - 1) >= 0 {
                            tmp_seat_vec.push(seats[i + 1][k - 1]);
                        }
                        if (k + 1) < row.len() {
                            tmp_seat_vec.push(seats[i + 1][k + 1]);
                        }
                    }
                    if (k as i32 - 1) >= 0 {
                        tmp_seat_vec.push(seats[i][k - 1]);
                    }
                    if (k + 1) < row.len() {
                        tmp_seat_vec.push(seats[i][k + 1]);
                    }
                    let mut count = 0;
                    for s in tmp_seat_vec {
                        if s == '#' {
                            count += 1;
                        }
                    }
                    if count == 0 {
                        new_seats[i][k] = '#'
                    }
                }
                &_ => {}
            }
        }
    }
    new_seats
}

fn update_seats_part2(seats: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_seats = seats.clone();
    for (i, row) in seats.iter().enumerate() {
        for (k, seat) in row.iter().enumerate() {
            match seat {
                &'#' => {
                    let mut tmp_seat_vec = Vec::new();
                    tmp_seat_vec.push(look_far(&seats, i, k, 1, 1));
                    tmp_seat_vec.push(look_far(&seats, i, k, -1, 1));
                    tmp_seat_vec.push(look_far(&seats, i, k, 1, -1));
                    tmp_seat_vec.push(look_far(&seats, i, k, -1, -1));
                    tmp_seat_vec.push(look_far(&seats, i, k, 1, 0));
                    tmp_seat_vec.push(look_far(&seats, i, k, 0, 1));
                    tmp_seat_vec.push(look_far(&seats, i, k, 0, -1));
                    tmp_seat_vec.push(look_far(&seats, i, k, -1, 0));
                    let mut count = 0;
                    for s in tmp_seat_vec {
                        if s == '#' {
                            count += 1;
                        }
                    }
                    if count >= 5 {
                        new_seats[i][k] = 'L'
                    }
                }
                &'L' => {
                    let mut tmp_seat_vec = Vec::new();
                    tmp_seat_vec.push(look_far(&seats, i, k, 1, 1));
                    tmp_seat_vec.push(look_far(&seats, i, k, -1, 1));
                    tmp_seat_vec.push(look_far(&seats, i, k, 1, -1));
                    tmp_seat_vec.push(look_far(&seats, i, k, -1, -1));
                    tmp_seat_vec.push(look_far(&seats, i, k, 1, 0));
                    tmp_seat_vec.push(look_far(&seats, i, k, 0, 1));
                    tmp_seat_vec.push(look_far(&seats, i, k, 0, -1));
                    tmp_seat_vec.push(look_far(&seats, i, k, -1, 0));
                    let mut count = 0;
                    for s in tmp_seat_vec {
                        if s == '#' {
                            count += 1;
                        }
                    }
                    if count == 0 {
                        new_seats[i][k] = '#'
                    }
                }
                &_ => {}
            }
        }
    }
    new_seats
}

fn look_far(seats: &Vec<Vec<char>>, start_x: usize, start_y: usize, x: i32, y: i32) -> char {
    if ((start_x as i32 + x) as usize) < seats.len()
        && start_x as i32 + x >= 0
        && ((start_y as i32 + y) as usize) < seats[0].len()
        && (start_y as i32 + y) >= 0
    {
        if seats[(start_x as i32 + x) as usize][(start_y as i32 + y) as usize] != '.' {
            seats[(start_x as i32 + x) as usize][(start_y as i32 + y) as usize]
        } else {
            look_far(
                seats,
                (start_x as i32 + x) as usize,
                (start_y as i32 + y) as usize,
                x,
                y,
            )
        }
    } else {
        '.'
    }
}

pub(crate) fn solve_day11_part2() -> u32 {
    let input_file = include_str!("day11.txt");
    let mut seats = input_file
        .lines()
        .collect_vec()
        .into_iter()
        .map(|s| s.chars().collect_vec())
        .collect_vec();

    loop {
        let new_seats = update_seats_part2(&seats);
        if seats == new_seats {
            break;
        }
        seats = new_seats;
    }

    count_seats(&seats)
}
