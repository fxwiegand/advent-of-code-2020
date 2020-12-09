use std::error::Error;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() -> Result<(), Box<dyn Error>> {
    // dbg!(day1::solve_day1());
    // dbg!(day1::solve_day1_part2());
    // dbg!(day2::solve_day());
    // dbg!(day2::solve_day2_part2());
    // dbg!(day3::solve_day3());
    // dbg!(day3::solve_day3_part2());
    // dbg!(day4::solve_day4());
    // dbg!(day4::solve_day4_part2());
    // dbg!(day5::solve_day5());
    // dbg!(day5::solve_day5_part2());
    // dbg!(day6::solve_day6());
    // dbg!(day6::solve_day6_part2());
    // dbg!(day7::solve_day7());
    // dbg!(day7::solve_day7_part2());
    // dbg!(day8::solve_day8());
    // dbg!(day8::solve_day8_part2());
    dbg!(day9::solve_day9());
    dbg!(day9::solve_day9_part2());
    Ok(())
}
