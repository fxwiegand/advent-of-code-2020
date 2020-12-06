use std::error::Error;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

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
    dbg!(day6::solve_day6());
    dbg!(day6::solve_day6_part2());
    Ok(())
}
