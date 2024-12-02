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
            Ok(day_num) => println!("Day {day_num} not implemented yet"),
            Err(e) => panic!("Could not parse argument {e:?}"),
        },
        _ => {
            help();
        }
    }
}
