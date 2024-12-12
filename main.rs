use std::env;

mod days;
use days::*;

fn help() {
    println!("usage: \n advent_of_code <num>: Solves the problem for day 1
    \n advent_of_code day<num> <input>: Tries to solve the problem for day <num> with the provided input text file");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => match args[1].parse() {
            Ok(1) => day1::solve(),
            Ok(2) => day2::solve(),
            Ok(3) => day3::solve(),
            Ok(4) => day4::solve(),
            Ok(5) => day5::solve(),
            Ok(6) => day6::solve(),
            Ok(7) => day7::solve(),
            Ok(8) => day8::solve(),
            Ok(9) => day9::solve(),
            Ok(10) => day10::solve(),
            Ok(11) => day11::solve(),
            Ok(12) => day12::solve(),
            Ok(day_num) => println!("Day {day_num} not implemented yet"),
            Err(e) => panic!("Could not parse argument {e:?}"),
        },
        _ => {
            help();
        }
    }
}
