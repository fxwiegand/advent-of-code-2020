use std::error::Error;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() -> Result<(), Box<dyn Error>> {
    // dbg!(day1::solve_day1());
    // dbg!(day1::solve_day1_part2());
    // dbg!(day2::solve_day());
    // dbg!(day2::solve_day2_part2());
    // dbg!(day3::solve_day3());
    // dbg!(day3::solve_day3_part2());
    dbg!(day4::solve_day4());
    dbg!(day4::solve_day4_part2());
    Ok(())
}
