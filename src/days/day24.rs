use itertools::Itertools;
use std::collections::HashMap;

pub(crate) fn solve_day24() -> u32 {
    let input_file = include_str!("../resources/day24.txt");
    let tiles = input_file
        .lines()
        .map(|c| c.chars().collect_vec())
        .collect_vec();

    let mut tile_directions = Vec::new();
    for tile in tiles {
        let mut tile_with_directions = Vec::new();
        let mut skip_next = false;
        for (index, _) in tile.iter().enumerate() {
            let mut dir = String::new();
            if skip_next {
                skip_next = false;
                continue;
            } else {
                match tile[index] {
                    's' => {
                        dir.push(tile[index]);
                        dir.push(tile[index + 1]);
                        skip_next = true;
                    }
                    'n' => {
                        dir.push(tile[index]);
                        dir.push(tile[index + 1]);
                        skip_next = true;
                    }
                    _ => {
                        dir.push(tile[index]);
                    }
                }
            }
            tile_with_directions.push(dir);
        }
        tile_directions.push(tile_with_directions);
    }

    let mut tile_map = HashMap::new();

    for tile in tile_directions {
        let west = tile.iter().filter(|s| s == &&"w").collect_vec().len() as i32;
        let east = tile.iter().filter(|s| s == &&"e").collect_vec().len() as i32;
        let south_east = tile.iter().filter(|s| s == &&"se").collect_vec().len() as i32;
        let north_east = tile.iter().filter(|s| s == &&"ne").collect_vec().len() as i32;
        let south_west = tile.iter().filter(|s| s == &&"sw").collect_vec().len() as i32;
        let north_west = tile.iter().filter(|s| s == &&"nw").collect_vec().len() as i32;

        let pos_west = (2 * west) + south_west + north_west;
        let pos_east = (2 * east) + south_east + north_east;
        let pos_north = north_east + north_west;
        let pos_south = south_west + south_east;

        let rec = tile_map
            .entry((pos_north - pos_south, pos_west - pos_east))
            .or_insert_with(|| 0);
        *rec += 1;
    }

    tile_map
        .values()
        .filter(|i| *i % 2 == 1)
        .collect_vec()
        .len() as u32
}
