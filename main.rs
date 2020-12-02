use std::error::Error;

mod day1;
mod day2;

fn main() -> Result<(), Box<dyn Error>> {
    // dbg!(day1::solve_day1());
    // dbg!(day1::solve_day1_part2());
    dbg!(day2::solve_day2());
    dbg!(day2::solve_day2_part2());
    Ok(())
}
