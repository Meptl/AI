use ::rand;
use ::rand::Rng;
use std::f64;

const STEP_SIZE: f64 = 0.25;

pub fn euclid_dist(point1: (f64, f64), point2: (f64, f64)) -> f64 {
    let delta_col = point2.0 - point1.0;
    let delta_row = point2.1 - point1.1;

    (delta_col * delta_col + delta_row * delta_row).sqrt()
}

pub struct World {
    tiles: Vec<Vec<u8>>,
    size: (usize, usize),
}

impl World {
    pub fn new(rows: usize, cols: usize, tiles: Vec<Vec<u8>>) -> World {
        World {
            tiles: tiles,
            size: (rows, cols),
        }
    }

    pub fn is_valid(&self, row: f64, col: f64) -> bool {
        // Out of bounds check. negatives overflow
        if row < 0.0 || row >= self.size.0 as f64 ||
            col < 0.0 || col >= self.size.1 as f64 {
            return false;
        }
        let mut pos = 0;

        // reverse because I stored my grid upside down
        for tile_row in self.tiles.iter().rev() {
            for tile_col in tile_row.iter() {
                if *tile_col == 1 {
                    let block_row = (pos / self.size.1) as f64;
                    let block_col = (pos % self.size.1) as f64;
                    if row > block_row && row < block_row + 1.0 &&
                        col > block_col && col < block_col + 1.0 {
                        return false;
                    }
                }
                pos += 1;
            }
        }
        true
    }

    /// Returns a random valid point on this world
    pub fn get_random_point(&self) -> (f64, f64) {
        let row = rand::random::<f64>() * (self.size.0) as f64;
        let col = rand::random::<f64>() * (self.size.1) as f64;
        if self.is_valid(row, col) {
            (row, col)
        } else {
            self.get_random_point()
        }
    }
}

pub struct Robot<'a> {
    pub pos: (f64, f64),
    pub goal: Option<(f64, f64)>,
    world: Option<&'a World>,
    pub moves: Vec<(f64, f64)>,
}

impl<'a> Robot<'a> {
    pub fn new(row: f64, col: f64) -> Robot<'a> {
        Robot {
            pos: (row, col),
            goal: None,
            world: None,
            moves: vec![(row, col)],
        }
    }

    fn move_vertical(&mut self, dest_row: f64) -> bool {
        let mut step_row = self.pos.0;
        if step_row < dest_row {
            while step_row < dest_row {
                step_row += STEP_SIZE;
                if !self.world.unwrap().is_valid(step_row, self.pos.1) {
                    return false;
                }
            }
        }
        else {
            while step_row > dest_row {
                step_row -= STEP_SIZE;
                if !self.world.unwrap().is_valid(step_row, self.pos.1) {
                    return false;
                }
            }
        }
        // destination has been checked for validity
        let cur_col = self.pos.1;
        self.set_pos((dest_row, cur_col));
        true
    }

    /// Attempts to move robot to location.
    /// Performs collision checks based with world.
    pub fn move_to(&mut self, dest_row: f64, dest_col: f64) -> bool {
        if self.world.is_none() {
            return false;
        }

        let world = self.world.unwrap();
        if !world.is_valid(dest_row, dest_col) {
            return false;
        }

        // row = y, col = x. When move_angle = 0, horizontal movement
        let delta_col = dest_col - self.pos.1;
        let delta_row = dest_row - self.pos.0;
        let move_angle = f64::atan(delta_row / delta_col);
        if move_angle.is_nan() {
            return self.move_vertical(dest_row);
        }

        let step_delta_row = f64::sin(move_angle) * STEP_SIZE;
        let step_delta_col = f64::cos(move_angle) * STEP_SIZE;

        let distance = euclid_dist(self.pos, (dest_row, dest_col));
        let mut step_row = self.pos.0;
        let mut step_col = self.pos.1;
        for _ in 0..(distance / STEP_SIZE) as usize {
            step_row += step_delta_row;
            step_col += step_delta_col;
            if !world.is_valid(step_row, step_col) {
                return false;
            }
        }

        self.set_pos((dest_row, dest_col));
        true
    }

    fn set_pos(&mut self, new_pos: (f64, f64)) {
        self.pos = new_pos;
        self.moves.push(new_pos);
    }

    pub fn set_world(&mut self, world: &'a World) -> Option<&'a World> {
        self.world = Some(world);
        Some(world)
    }

    pub fn set_goal(&mut self, row: f64, col: f64) {
        self.goal = Some((row, col))
    }

    pub fn is_done(&self) -> bool {
        if self.goal.is_none() {
            return false;
        }
        euclid_dist(self.pos, self.goal.unwrap()) < 1.0
    }
}

impl<'a> Clone for Robot<'a> {
    fn clone(&self) -> Self {
        Robot {
            pos: self.pos,
            goal: self.goal,
            world: self.world,
            moves: self.moves.clone(),
        }
    }
}
