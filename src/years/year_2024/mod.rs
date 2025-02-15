use super::super::{dir::*, library::*, map::*, pos::*};
use super::{AdventDay, DayNum, Year};
use std::collections::HashMap;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub struct Year2024 {
    days: HashMap<DayNum, Box<dyn AdventDay>>,
}

impl Year2024 {
    pub fn new() -> Year2024 {
        let mut days: HashMap<DayNum, Box<dyn AdventDay>> = HashMap::new();
        days.insert(DayNum::new(1).unwrap(), Box::new(day1::Day1 {}));
        days.insert(DayNum::new(2).unwrap(), Box::new(day2::Day2 {}));
        days.insert(DayNum::new(3).unwrap(), Box::new(day3::Day3 {}));
        days.insert(DayNum::new(4).unwrap(), Box::new(day4::Day4 {}));
        days.insert(DayNum::new(5).unwrap(), Box::new(day5::Day5 {}));
        days.insert(DayNum::new(6).unwrap(), Box::new(day6::Day6 {}));
        days.insert(DayNum::new(7).unwrap(), Box::new(day7::Day7 {}));
        days.insert(DayNum::new(8).unwrap(), Box::new(day8::Day8 {}));
        days.insert(DayNum::new(9).unwrap(), Box::new(day9::Day9 {}));
        days.insert(DayNum::new(10).unwrap(), Box::new(day10::Day10 {}));
        days.insert(DayNum::new(11).unwrap(), Box::new(day11::Day11 {}));
        days.insert(DayNum::new(12).unwrap(), Box::new(day12::Day12 {}));
        days.insert(DayNum::new(13).unwrap(), Box::new(day13::Day13 {}));
        days.insert(DayNum::new(14).unwrap(), Box::new(day14::Day14 {}));
        days.insert(DayNum::new(15).unwrap(), Box::new(day15::Day15 {}));
        days.insert(DayNum::new(16).unwrap(), Box::new(day16::Day16 {}));
        days.insert(DayNum::new(17).unwrap(), Box::new(day17::Day17 {}));
        days.insert(DayNum::new(18).unwrap(), Box::new(day18::Day18 {}));
        days.insert(DayNum::new(19).unwrap(), Box::new(day19::Day19 {}));
        days.insert(DayNum::new(20).unwrap(), Box::new(day20::Day20 {}));
        days.insert(DayNum::new(21).unwrap(), Box::new(day21::Day21 {}));
        days.insert(DayNum::new(22).unwrap(), Box::new(day22::Day22 {}));
        days.insert(DayNum::new(23).unwrap(), Box::new(day23::Day23 {}));
        days.insert(DayNum::new(24).unwrap(), Box::new(day24::Day24 {}));
        days.insert(DayNum::new(25).unwrap(), Box::new(day25::Day25 {}));
        // days.insert(Day::Day2, Box::new(day2::Day2{}));

        Year2024 { days }
    }
}

impl Year for Year2024 {
    fn solve_day(&self, day: DayNum) {
        if let Some(challenge) = self.days.get(&day) {
            challenge.solve();
        } else {
            println!("Day {day:?} is not implemented for year 2024");
        }
    }
}
