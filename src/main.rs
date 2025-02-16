use std::env;

mod library;
pub use library::*;
use years::{year_2023, year_2024, DayNum, Year};
mod years;

fn help() {
    println!(
        "usage: \n advent_of_code <num1> <num2>: Solves the year <num1> problem for day <num2>"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        help();
        return;
    }

    let year_num = args[1].parse::<i32>();
    let day_num = args[2].parse::<i32>().unwrap_or(0);

    let day = DayNum::new(day_num).expect("Day number should be between 1 and 25!");

    match year_num {
        Ok(2023) => { year_2023::Year2023 {}.solve_day(day); }
        Ok(2024) => { year_2024::Year2024 {}.solve_day(day); }
        Ok(year) => panic!("Year {year} not implemented yet!"),
        Err(_) => panic!("Year argument should be a number!"),
    }
}
