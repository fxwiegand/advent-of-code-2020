#![feature(destructuring_assignment)]
use clap::Clap;
use days::{
    day1, day10, day11, day12, day13, day14, day15, day16, day2, day3, day4, day5, day6, day7,
    day8, day9,
};
use std::error::Error;

mod days;

fn main() -> Result<(), Box<dyn Error>> {
    let opts: Opts = Opts::parse();

    let result = match (opts.day, opts.part_two) {
        (1, false) => day1::solve_day1().to_string(),
        (1, true) => day1::solve_day1_part2().to_string(),
        (2, false) => day2::solve_day2().to_string(),
        (2, true) => day2::solve_day2_part2().to_string(),
        (3, false) => day3::solve_day3().to_string(),
        (3, true) => day3::solve_day3_part2().to_string(),
        (4, false) => day4::solve_day4().to_string(),
        (4, true) => day4::solve_day4_part2().to_string(),
        (5, false) => day5::solve_day5().to_string(),
        (5, true) => day5::solve_day5_part2().to_string(),
        (6, false) => day6::solve_day6().to_string(),
        (6, true) => day6::solve_day6_part2().to_string(),
        (7, false) => day7::solve_day7().to_string(),
        (7, true) => day7::solve_day7_part2().to_string(),
        (8, false) => day8::solve_day8().to_string(),
        (8, true) => day8::solve_day8_part2().to_string(),
        (9, false) => day9::solve_day9().to_string(),
        (9, true) => day9::solve_day9_part2().to_string(),
        (10, false) => day10::solve_day10().to_string(),
        (10, true) => day10::solve_day10_part2().to_string(),
        (11, false) => day11::solve_day11().to_string(),
        (11, true) => day11::solve_day11_part2().to_string(),
        (12, false) => day12::solve_day12().to_string(),
        (12, true) => day12::solve_day12_part2().to_string(),
        (13, false) => day13::solve_day13().to_string(),
        (13, true) => day13::solve_day13_part2().to_string(),
        (14, false) => day14::solve_day14().to_string(),
        (14, true) => day14::solve_day14_part2().to_string(),
        (15, false) => day15::solve_day15().to_string(),
        (15, true) => day15::solve_day15_part2().to_string(),
        (16, false) => day16::solve_day16().to_string(),
        (16, true) => day16::solve_day16_part2().to_string(),
        _ => unimplemented!(),
    };

    if opts.part_two {
        println!(
            "The solution for the second part of day {} is {}.",
            opts.day, result
        );
    } else {
        println!("The solution for day {} is {}.", opts.day, result);
    }

    Ok(())
}

#[derive(Clap)]
#[clap(version = "1.0", author = "Felix W. <fxwiegand@wgdnet.de>")]
struct Opts {
    /// The day of advent of code.
    day: usize,
    /// Whether you want the solution for part 1 or 2.
    #[clap(short, long)]
    part_two: bool,
}
