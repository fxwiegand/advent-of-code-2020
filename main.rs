use std::error::Error;

mod day1;

fn main() -> Result<(), Box<dyn Error>> {
    dbg!(day1::solve_day1());
    dbg!(day1::solve_day1_part2());
    Ok(())
}