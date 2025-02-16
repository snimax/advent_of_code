use super::super::{dir::*, library::*, map::*, pos::*};
use super::{AdventDay, DayNum, Year};

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

pub struct Year2024 {}

impl Year for Year2024 {
    fn solve_day(&self, day: DayNum) {
        match day {
            DayNum(1) => day1::Day1 {}.solve(),
            DayNum(2) => day2::Day2 {}.solve(),
            DayNum(3) => day3::Day3 {}.solve(),
            DayNum(4) => day4::Day4 {}.solve(),
            DayNum(5) => day5::Day5 {}.solve(),
            DayNum(6) => day6::Day6 {}.solve(),
            DayNum(7) => day7::Day7 {}.solve(),
            DayNum(8) => day8::Day8 {}.solve(),
            DayNum(9) => day9::Day9 {}.solve(),
            DayNum(10) => day10::Day10 {}.solve(),
            DayNum(11) => day11::Day11 {}.solve(),
            DayNum(12) => day12::Day12 {}.solve(),
            DayNum(13) => day13::Day13 {}.solve(),
            DayNum(14) => day14::Day14 {}.solve(),
            DayNum(15) => day15::Day15 {}.solve(),
            DayNum(16) => day16::Day16 {}.solve(),
            DayNum(17) => day17::Day17 {}.solve(),
            DayNum(18) => day18::Day18 {}.solve(),
            DayNum(19) => day19::Day19 {}.solve(),
            DayNum(20) => day20::Day20 {}.solve(),
            DayNum(21) => day21::Day21 {}.solve(),
            DayNum(22) => day22::Day22 {}.solve(),
            DayNum(23) => day23::Day23 {}.solve(),
            DayNum(24) => day24::Day24 {}.solve(),
            DayNum(25) => day25::Day25 {}.solve(),
            _ => println!("Day {day:?} is not implemented for year 2024"),
        }
    }
}
