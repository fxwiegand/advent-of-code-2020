use itertools::Itertools;
use std::str::FromStr;

pub(crate) fn solve_day12() -> i32 {
    let input_file = include_str!("../resources/day12.txt");
    let navigation = input_file
        .lines()
        .collect_vec()
        .into_iter()
        .map(|s| {
            let (ins, n) = s.split_at(1);
            (ins.chars().collect_vec()[0], i32::from_str(n).unwrap())
        })
        .collect_vec();

    let mut direction = 'E';
    let mut position_n: i32 = 0;
    let mut position_e: i32 = 0;

    for (instruction, num) in navigation {
        match instruction {
            'N' => {
                position_n += num;
            }
            'S' => {
                position_n -= num;
            }
            'E' => {
                position_e += num;
            }
            'W' => {
                position_e -= num;
            }
            'F' => match direction {
                'N' => {
                    position_n += num;
                }
                'S' => {
                    position_n -= num;
                }
                'E' => {
                    position_e += num;
                }
                'W' => {
                    position_e -= num;
                }
                _ => unreachable!(),
            },
            'R' => match num {
                90 => match direction {
                    'N' => {
                        direction = 'E';
                    }
                    'S' => {
                        direction = 'W';
                    }
                    'E' => {
                        direction = 'S';
                    }
                    'W' => {
                        direction = 'N';
                    }
                    _ => unreachable!(),
                },
                180 => match direction {
                    'N' => {
                        direction = 'S';
                    }
                    'S' => {
                        direction = 'N';
                    }
                    'E' => {
                        direction = 'W';
                    }
                    'W' => {
                        direction = 'E';
                    }
                    _ => unreachable!(),
                },
                270 => match direction {
                    'N' => {
                        direction = 'W';
                    }
                    'S' => {
                        direction = 'E';
                    }
                    'E' => {
                        direction = 'N';
                    }
                    'W' => {
                        direction = 'S';
                    }
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            'L' => match num {
                270 => match direction {
                    'N' => {
                        direction = 'E';
                    }
                    'S' => {
                        direction = 'W';
                    }
                    'E' => {
                        direction = 'S';
                    }
                    'W' => {
                        direction = 'N';
                    }
                    _ => unreachable!(),
                },
                180 => match direction {
                    'N' => {
                        direction = 'S';
                    }
                    'S' => {
                        direction = 'N';
                    }
                    'E' => {
                        direction = 'W';
                    }
                    'W' => {
                        direction = 'E';
                    }
                    _ => unreachable!(),
                },
                90 => match direction {
                    'N' => {
                        direction = 'W';
                    }
                    'S' => {
                        direction = 'E';
                    }
                    'E' => {
                        direction = 'N';
                    }
                    'W' => {
                        direction = 'S';
                    }
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    i32::abs(position_n) + i32::abs(position_e)
}

pub(crate) fn solve_day12_part2() -> i32 {
    let input_file = include_str!("../resources/day12.txt");
    let navigation = input_file
        .lines()
        .collect_vec()
        .into_iter()
        .map(|s| {
            let (ins, n) = s.split_at(1);
            (ins.chars().collect_vec()[0], i32::from_str(n).unwrap())
        })
        .collect_vec();

    let mut position_n: i32 = 0;
    let mut position_e: i32 = 0;
    let mut wp_position_n: i32 = 1;
    let mut wp_position_e: i32 = 10;

    for (instruction, num) in navigation {
        match instruction {
            'N' => {
                wp_position_n += num;
            }
            'S' => {
                wp_position_n -= num;
            }
            'E' => {
                wp_position_e += num;
            }
            'W' => {
                wp_position_e -= num;
            }
            'F' => {
                position_n += num * wp_position_n;
                position_e += num * wp_position_e;
            }
            'R' => match num {
                90 => {
                    let old_wp_n = wp_position_n;
                    let old_wp_e = wp_position_e;
                    wp_position_e = old_wp_n;
                    wp_position_n = -old_wp_e;
                }
                180 => {
                    let old_wp_n = wp_position_n;
                    let old_wp_e = wp_position_e;
                    wp_position_e = -old_wp_e;
                    wp_position_n = -old_wp_n;
                }
                270 => {
                    let old_wp_n = wp_position_n;
                    let old_wp_e = wp_position_e;
                    wp_position_e = -old_wp_n;
                    wp_position_n = old_wp_e;
                }
                _ => unreachable!(),
            },
            'L' => match num {
                270 => {
                    let old_wp_n = wp_position_n;
                    let old_wp_e = wp_position_e;
                    wp_position_e = old_wp_n;
                    wp_position_n = -old_wp_e;
                }
                180 => {
                    let old_wp_n = wp_position_n;
                    let old_wp_e = wp_position_e;
                    wp_position_e = -old_wp_e;
                    wp_position_n = -old_wp_n;
                }
                90 => {
                    let old_wp_n = wp_position_n;
                    let old_wp_e = wp_position_e;
                    wp_position_e = -old_wp_n;
                    wp_position_n = old_wp_e;
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    i32::abs(position_n) + i32::abs(position_e)
}
