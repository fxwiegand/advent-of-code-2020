use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub(crate) fn solve_day20() -> u64 {
    let input_file = include_str!("../resources/day20.txt");
    let tmp_tiles = input_file.split("\n\n").collect_vec();

    let mut tiles = HashMap::new();
    for tile in tmp_tiles {
        let mut t = HashSet::new();
        let id = u64::from_str(
            tile.lines().take(1).collect_vec()[0]
                .split_whitespace()
                .collect_vec()[1]
                .trim_end_matches(':'),
        )
        .unwrap();
        let first_edge = tile.lines().skip(1).take(1).collect_vec()[0]
            .chars()
            .collect_vec();
        let fourth_edge = tile.lines().skip(10).take(1).collect_vec()[0]
            .chars()
            .collect_vec();
        let second_edge = tile
            .lines()
            .skip(1)
            .map(|s| s.chars().collect_vec()[0])
            .collect_vec();
        let third_edge = tile
            .lines()
            .skip(1)
            .map(|s| s.chars().collect_vec()[9])
            .collect_vec();
        t.insert(first_edge);
        t.insert(second_edge);
        t.insert(third_edge);
        t.insert(fourth_edge);
        tiles.insert(id, t);
    }

    let mut result = 1;
    for (id, edges) in &tiles {
        let mut tile_fits_at_x_edges = 0;
        for edge in edges {
            let mut count_fitting_edges = 0;
            for (id2, edges_to_check) in &tiles {
                if id != id2 {
                    if edges_to_check.contains(edge)
                        || edges_to_check
                            .contains(edge.into_iter().rev().map(|c| *c).collect_vec().as_slice())
                    {
                        count_fitting_edges += 1;
                    }
                }
            }
            if count_fitting_edges > 0 {
                tile_fits_at_x_edges += 1;
            }
        }
        if tile_fits_at_x_edges == 2 {
            println!("Found tile!");
            result *= id;
        }
    }
    result
}
