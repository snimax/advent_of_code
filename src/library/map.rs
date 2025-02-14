use super::dir::*;
use super::pos::*;

#[derive(Clone)]
pub struct Map<T: Copy> {
    pub map: Vec<Vec<T>>,
    pub size_x: usize,
    pub size_y: usize,
}

impl<T: Copy> Map<T> {
    pub fn next(&self, curr_pos: &Pos, dir: &Dir) -> Option<T> {
        let new_pos = Pos {
            x: curr_pos.x + dir.x,
            y: curr_pos.y + dir.y,
        };
        match self.valid_pos(&new_pos) {
            false => None,
            true => Some(self.get(&new_pos)),
        }
    }

    pub fn get(&self, pos: &Pos) -> T {
        self.map[pos.y as usize][pos.x as usize]
    }

    pub fn set(&mut self, pos: &Pos, val: T) {
        self.map[pos.y as usize][pos.x as usize] = val;
    }

    pub fn valid_pos(&self, pos: &Pos) -> bool {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.size_x as i32 || pos.y >= self.size_y as i32 {
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
            if self.valid_pos(&neighbor_pos) && self.get(&neighbor_pos) == *cmp_val {
                valid_neighbors.push(neighbor_pos);
            }
        }

        valid_neighbors
    }
}
