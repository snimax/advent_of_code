use std::collections::HashMap;
use std::env;

mod library;
pub use library::*;
use years::{DayNum, Year, YearNum};
mod years;

fn help() {
    println!(
        "usage: \n advent_of_code <num1> <num2>: Solves the year <num1> problem for day <num2>"
    );
    // "\n advent_of_code day<num> <input>: Tries to solve the problem for day <num> with the provided input text file");
}

fn main() {
    let mut years = HashMap::new();
    years.insert(
        YearNum::new(2024).unwrap(),
        years::year_2024::Year2024::new(),
    );

    let args: Vec<String> = env::args().collect();

    if args.len() >= 3 {
        if let Ok(Some(year_num)) = args[1].parse().map(YearNum::new) {
            if let Ok(Some(day_num)) = args[2].parse().map(DayNum::new) {
                if let Some(year) = years.get(&year_num) {
                    year.solve_day(day_num);
                } else {
                    println!("Year {} not implemented yet!", args[1]);
                }
            } else {
                println!("Could not parse a valid day from {}!\nValid days are between 1 and 25!", args[2]);
            }
        } else {
            println!("Could not parse a valid year from '{}'!", args[1]);
        }
    } else {
        help();
    }
}
