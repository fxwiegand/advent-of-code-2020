use itertools::Itertools;

pub(crate) fn solve_day17() -> u64 {
    let input_file = include_str!("../resources/day17.txt");
    let input_grid = input_file
        .lines()
        .map(|n| n.chars().collect_vec())
        .collect_vec();

    let grid_size = 50;
    let mut grid = vec![vec![vec!['.'; grid_size]; grid_size]; grid_size];
    for (y, line) in input_grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            grid[25][21 + y][21 + x] = *c;
        }
    }

    for _ in 0..6 {
        let mut next_grid = grid.clone();
        for (z, zz) in grid.iter().enumerate().skip(1).take(grid.len() - 2) {
            for (y, yy) in zz.iter().enumerate().skip(1).take(zz.len() - 2) {
                for (x, _) in yy.iter().enumerate().skip(1).take(yy.len() - 2) {
                    next_grid[z][y][x] = match grid[z][y][x] {
                        '#' => {
                            let dims: Vec<i32> = vec![1, 0, -1];
                            let mut count_active = 0;
                            for ((z1, y1), x1) in dims
                                .iter()
                                .cartesian_product(&dims)
                                .cartesian_product(&dims)
                            {
                                if !(*z1 == 0 && *y1 == 0 && *x1 == 0) {
                                    if grid[(z as i32 + *z1) as usize][(y as i32 + *y1) as usize]
                                        [(x as i32 + *x1) as usize]
                                        == '#'
                                    {
                                        count_active += 1;
                                    }
                                }
                            }
                            if count_active == 2 || count_active == 3 {
                                '#'
                            } else {
                                '.'
                            }
                        }
                        '.' => {
                            let dims = vec![1, 0, -1];
                            let mut count_active = 0;
                            for ((z1, y1), x1) in dims
                                .iter()
                                .cartesian_product(&dims)
                                .cartesian_product(&dims)
                            {
                                if !(*z1 == 0 && *y1 == 0 && *x1 == 0) {
                                    if grid[(z as i32 + *z1) as usize][(y as i32 + *y1) as usize]
                                        [(x as i32 + *x1) as usize]
                                        == '#'
                                    {
                                        count_active += 1;
                                    }
                                }
                            }
                            if count_active == 3 {
                                '#'
                            } else {
                                '.'
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
        grid = next_grid.clone();
    }

    let mut res = 0;

    for z in grid {
        for y in z {
            for x in y {
                if x == '#' {
                    res += 1;
                }
            }
        }
    }

    res
}

pub(crate) fn solve_day17_part2() -> u64 {
    let input_file = include_str!("../resources/day17.txt");
    let input_grid = input_file
        .lines()
        .map(|n| n.chars().collect_vec())
        .collect_vec();

    let grid_size = 52;
    let mut grid = vec![vec![vec![vec!['.'; grid_size]; grid_size]; grid_size]; grid_size];
    for (y, line) in input_grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            grid[26][26][21 + y][21 + x] = *c;
        }
    }

    for _ in 0..6 {
        let mut next_grid = grid.clone();
        for (w, ww) in grid.iter().enumerate().skip(1).take(grid.len() - 2) {
            for (z, zz) in ww.iter().enumerate().skip(1).take(ww.len() - 2) {
                for (y, yy) in zz.iter().enumerate().skip(1).take(zz.len() - 2) {
                    for (x, _) in yy.iter().enumerate().skip(1).take(yy.len() - 2) {
                        next_grid[w][z][y][x] = match grid[w][z][y][x] {
                            '#' => {
                                let dims: Vec<i32> = vec![1, 0, -1];
                                let mut count_active = 0;
                                for (((w1, z1), y1), x1) in dims
                                    .iter()
                                    .cartesian_product(&dims)
                                    .cartesian_product(&dims)
                                    .cartesian_product(&dims)
                                {
                                    if !(*w1 == 0 && *z1 == 0 && *y1 == 0 && *x1 == 0) {
                                        if grid[(w as i32 + *w1) as usize]
                                            [(z as i32 + *z1) as usize]
                                            [(y as i32 + *y1) as usize]
                                            [(x as i32 + *x1) as usize]
                                            == '#'
                                        {
                                            count_active += 1;
                                        }
                                    }
                                }
                                if count_active == 2 || count_active == 3 {
                                    '#'
                                } else {
                                    '.'
                                }
                            }
                            '.' => {
                                let dims = vec![1, 0, -1];
                                let mut count_active = 0;
                                for (((w1, z1), y1), x1) in dims
                                    .iter()
                                    .cartesian_product(&dims)
                                    .cartesian_product(&dims)
                                    .cartesian_product(&dims)
                                {
                                    if !(*w1 == 0 && *z1 == 0 && *y1 == 0 && *x1 == 0) {
                                        if grid[(w as i32 + *w1) as usize]
                                            [(z as i32 + *z1) as usize]
                                            [(y as i32 + *y1) as usize]
                                            [(x as i32 + *x1) as usize]
                                            == '#'
                                        {
                                            count_active += 1;
                                        }
                                    }
                                }
                                if count_active == 3 {
                                    '#'
                                } else {
                                    '.'
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                }
            }
        }
        grid = next_grid.clone();
    }

    let mut res = 0;

    for w in grid {
        for z in w {
            for y in z {
                for x in y {
                    if x == '#' {
                        res += 1;
                    }
                }
            }
        }
    }

    res
}
