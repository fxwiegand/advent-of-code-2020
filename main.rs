use std::error::Error;

mod day1;
mod day2;
mod day3;

fn main() -> Result<(), Box<dyn Error>> {
    // dbg!(day1::solve_day1());
    // dbg!(day1::solve_day1_part2());
    // dbg!(day2::solve_day());
    // dbg!(day2::solve_day2_part2());
    dbg!(day3::solve_day3());
    dbg!(day3::solve_day3_part2());
    Ok(())
}
