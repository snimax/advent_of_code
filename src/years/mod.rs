use crate::library::{parse_file, parse_lines};
pub mod year_2024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DayNum(i32);
impl DayNum {
    pub fn new(i: i32) -> Option<DayNum> {
        if (1..=25).contains(&i) {
            Some(DayNum(i))
        } else {
            None
        }
    }
}

pub trait Year {
    fn solve_day(&self, day: DayNum);
}

pub trait AdventDay {
    fn solve(&self) {
        panic!("Day not implemented yet!");
    }

    fn get_input(&self) -> Vec<String> {
        if let Ok(line_string) = parse_file(self.get_input_path()) {
            parse_lines(&line_string)
        } else {
            panic!("Could not get/parse input");
        }
    }

    fn get_input_path(&self) -> &str {
        panic!("get_input_path not implemented for this day!");
    }
}
