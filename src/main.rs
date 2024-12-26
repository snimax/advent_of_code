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
            Ok(13) => day13::solve(),
            Ok(14) => day14::solve(),
            Ok(15) => day15::solve(),
            Ok(16) => day16::solve(),
            Ok(17) => day17::solve(),
            Ok(18) => day18::solve(),
            Ok(19) => day19::solve(),
            Ok(20) => day20::solve(),
            Ok(21) => day21::solve(),
            Ok(22) => day22::solve(),
            Ok(23) => day23::solve(),
            Ok(24) => day24::solve(),
            Ok(25) => day25::solve(),
            Ok(day_num) => println!("Day {day_num} not implemented yet"),
            Err(e) => panic!("Could not parse argument {e:?}"),
        },
        _ => {
            help();
        }
    }
}
