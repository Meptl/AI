pub enum Action {
    North,
    South,
    East,
    West,
    Vacuum,
}

pub enum Tile {
    Clean,
    Dirty,
    Blocked
}

pub struct World {
    tiles: Vec<Tile>,
    x: usize,
    y: usize,
    rob: usize,
}

impl World {
    pub fn new(tiles: Vec<Tile>, x: usize, y: usize, rob: usize) -> World {
        World {
            tiles: tiles,
            x: x,
            y: y,
            rob: rob,
        }
    }
}
