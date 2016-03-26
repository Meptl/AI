use std::fmt;

pub enum Algorithm {
    DepthFirst,
    DepthFirstIteratedDeepening,
    BreadthFirst,
    AStar
}

pub enum Action {
    North,
    South,
    East,
    West,
    Vacuum,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Action::North => "N",
            Action::South => "S",
            Action::East => "E",
            Action::West => "W",
            Action::Vacuum => "V",
        })
    }
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

    pub fn path_find(world: &World, _: Algorithm) -> Vec<Action> {
        vec![Action::North]
    }
}
