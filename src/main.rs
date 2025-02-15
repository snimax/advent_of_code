use std::env;

mod library;
pub use library::*;
mod years;

fn help() {
    println!("usage: \n advent_of_code <num>: Solves the problem for day 1
    \n advent_of_code day<num> <input>: Tries to solve the problem for day <num> with the provided input text file");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => match args[1].parse() {
            Ok(1) => years::year_2024::day1::solve(),
            Ok(2) => years::year_2024::day2::solve(),
            Ok(3) => years::year_2024::day3::solve(),
            Ok(4) => years::year_2024::day4::solve(),
            Ok(5) => years::year_2024::day5::solve(),
            Ok(6) => years::year_2024::day6::solve(),
            Ok(7) => years::year_2024::day7::solve(),
            Ok(8) => years::year_2024::day8::solve(),
            Ok(9) => years::year_2024::day9::solve(),
            Ok(10) => years::year_2024::day10::solve(),
            Ok(11) => years::year_2024::day11::solve(),
            Ok(12) => years::year_2024::day12::solve(),
            Ok(13) => years::year_2024::day13::solve(),
            Ok(14) => years::year_2024::day14::solve(),
            Ok(15) => years::year_2024::day15::solve(),
            Ok(16) => years::year_2024::day16::solve(),
            Ok(17) => years::year_2024::day17::solve(),
            Ok(18) => years::year_2024::day18::solve(),
            Ok(19) => years::year_2024::day19::solve(),
            Ok(20) => years::year_2024::day20::solve(),
            Ok(21) => years::year_2024::day21::solve(),
            Ok(22) => years::year_2024::day22::solve(),
            Ok(23) => years::year_2024::day23::solve(),
            Ok(24) => years::year_2024::day24::solve(),
            Ok(25) => years::year_2024::day25::solve(),
            Ok(day_num) => println!("Day {day_num} not implemented yet"),
            Err(e) => panic!("Could not parse argument {e:?}"),
        },
        _ => {
            help();
        }
    }
}
