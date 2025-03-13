use super::dir::*;
use super::pos::*;

pub struct Map<T> {
    pub map: Vec<T>,
    size_x: usize,
    size_y: usize,
    transposed: bool,
}

impl<T> Map<T> {
    pub fn new<F>(lines: &[String], mut func: F) -> Map<T>
    where
        F: FnMut(char, &Pos) -> T,
    {
        let size_y = lines.len();
        let size_x = lines[0].len();
        let mut map = Vec::with_capacity(size_y * size_x);

        for (row, line) in lines.iter().enumerate() {
            for (col, char) in line.chars().enumerate() {
                map.push(func(
                    char,
                    &Pos {
                        x: col as i32,
                        y: row as i32,
                    },
                ));
            }
        }

        Map {
            map,
            size_x,
            size_y,
            transposed: false,
        }
    }

    pub fn rows(&self) -> usize {
        if self.transposed {
            self.size_x
        } else {
            self.size_y
        }
    }

    pub fn cols(&self) -> usize {
        if self.transposed {
            self.size_y
        } else {
            self.size_x
        }
    }

    fn transplate_pos_to_index(&self, pos: &Pos) -> usize {
        if !self.transposed {
            (pos.y * self.cols() as i32 + pos.x) as usize
        } else {
            (pos.x * self.rows() as i32 + pos.y) as usize
        }
    }

    pub fn next(&self, curr_pos: &Pos, dir: &Dir) -> Option<&T> {
        let new_pos = Pos {
            x: curr_pos.x + dir.x,
            y: curr_pos.y + dir.y,
        };
        match self.valid_pos(&new_pos) {
            false => None,
            true => Some(self.get(&new_pos)),
        }
    }

    pub fn get(&self, pos: &Pos) -> &T {
        &self.map[self.transplate_pos_to_index(pos)]
    }

    pub fn set(&mut self, pos: &Pos, val: T) {
        let index = self.transplate_pos_to_index(pos);
        self.map[index] = val;
    }

    pub fn valid_pos(&self, pos: &Pos) -> bool {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.cols() as i32 || pos.y >= self.rows() as i32 {
            return false;
        }
        true
    }

    pub fn get_neighbors_cmp(&self, pos: &Pos, cmp_val: &T) -> Vec<Pos>
    where
        T: std::cmp::PartialEq,
    {
        let mut valid_neighbors = Vec::new();

        for &dir in DIRECTIONS.iter() {
            let neighbor_pos = pos + dir;
            if self.valid_pos(&neighbor_pos) && self.get(&neighbor_pos) == cmp_val {
                valid_neighbors.push(neighbor_pos);
            }
        }

        valid_neighbors
    }

    pub fn transpose(&mut self) {
        self.transposed = !self.transposed;
    }

    pub fn print<F>(&self, mut func: F)
    where
        F: FnMut(&T) -> char,
    {
        for y in 0..self.rows() as i32 {
            for x in 0..self.cols() as i32 {
                print!("{}", func(self.get(&Pos { x, y })));
            }
            println!()
        }
    }
}

impl<T: Clone> Clone for Map<T> {
    fn clone(&self) -> Self {
        Map {
            map: self.map.clone(),
            size_x: self.size_x,
            size_y: self.size_y,
            transposed: self.transposed,
        }
    }
}

#[test]
fn transpose_map() {
    let mut map = Map::new(&["AB".to_string(), "CD".to_string()], |c, _| c);
    assert!(*map.get(&Pos { x: 1, y: 0 }) == 'B');
    assert!(*map.get(&Pos { x: 0, y: 1 }) == 'C');
    map.transpose();
    assert!(*map.get(&Pos { x: 0, y: 1 }) == 'B');
    assert!(*map.get(&Pos { x: 1, y: 0 }) == 'C');

    let mut map = Map::new(&["ABC".to_string(), "DEF".to_string()], |c, _| c);
    assert!(*map.get(&Pos { x: 2, y: 0 }) == 'C');
    assert!(*map.get(&Pos { x: 0, y: 1 }) == 'D');
    map.transpose();
    assert!(*map.get(&Pos { x: 0, y: 2 }) == 'C');
    assert!(*map.get(&Pos { x: 1, y: 0 }) == 'D');
}
