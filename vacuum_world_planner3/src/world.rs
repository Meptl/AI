use std::fmt;

pub enum Algorithm {
    DepthFirst,
    DepthFirstIteratedDeepening,
    BreadthFirst,
    AStar
}

pub struct World {
    tiles: Vec<Tile>,
    x: usize,
    y: usize,
    rob: usize,
    dirt: usize
}

#[derive(PartialEq)]
pub enum Tile {
    Clean,
    Dirty,
    Blocked
}

#[derive(PartialEq, Copy, Clone)]
pub enum Action {
    North,
    South,
    East,
    West,
    Vacuum,
}

/// Order of Action testing is V, W, S, E, N.
struct WorldActions<'a> {
    world: &'a World,
    next_action: Option<Action>,
}


impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Action::Vacuum => "V",
            Action::North => "N",
            Action::South => "S",
            Action::East => "E",
            Action::West => "W"
        })
    }
}

impl<'a> Iterator for WorldActions<'a> {
    type Item = Action;

    fn next(&mut self) -> Option<Action> {
        if self.next_action == None {
            return None;
        }

        let prev_action = self.next_action.unwrap();

        // Assign next action
        self.next_action = match prev_action {
            Action::Vacuum => Some(Action::West),
            Action::West => Some(Action::South),
            Action::South => Some(Action::East),
            Action::East => Some(Action::North),
            Action::North => None,
        };

        if self.world.can_perform(prev_action) {
            Some(prev_action)
        } else {
            self.next()
        }
    }
}

impl World {
    pub fn new(tiles: Vec<Tile>, x: usize, y: usize,
               rob: usize) -> World {
        let dirt_count = tiles.iter().filter(|&x| *x == Tile::Dirty).count();
        World {
            tiles: tiles,
            x: x,
            y: y,
            rob: rob,
            dirt: dirt_count,
        }
    }

    /// Finds a path to a goal state from the given world using the algorithm.
    pub fn path_find(world: World, alg: Algorithm) -> Vec<Action> {
        match alg {
            Algorithm::DepthFirst => World::depth_first(world, Vec::new()),
            Algorithm::DepthFirstIteratedDeepening => vec![],
            Algorithm::BreadthFirst => vec![],
            Algorithm::AStar => vec![],
        }
    }

    /// Is the world clean?
    pub fn is_solved(&self) -> bool {
        self.dirt == 0
    }

    /// Returns an iterator over possible actions.
    fn actions(&self) -> WorldActions {
        WorldActions { world: &self, next_action: Some(Action::Vacuum) }
    }

    /// Determines if an action can be performed.
    fn can_perform(&self, action: Action) -> bool {
        match action {
            Action::Vacuum => self.tiles[self.rob] == Tile::Dirty,
            Action::North => {
                // Check if top edge of world
                if self.rob < self.x {
                    false
                } else {
                    self.tiles[self.rob - self.x] != Tile::Blocked
                }
            }
            Action::South => {
                // Check if bottom edge of world
                if self.rob + self.x > self.tiles.len() {
                    false
                } else {
                    self.tiles[self.rob + self.x] != Tile::Blocked
                }
            }
            Action::East => {
                // Check if right edge of world
                if (self.rob + 1) % self.x == 0 {
                    false
                } else {
                    self.tiles[self.rob + 1] != Tile::Blocked
                }
            }
            Action::West => {
                // Check if left edge of world
                if self.rob % self.x == 0 {
                    false
                } else {
                    self.tiles[self.rob - 1] != Tile::Blocked
                }
            }
        }
    }

    /// Returns a world modified by the action. Assumes action can be performed.
    fn do_action(mut self, action: Action) -> World {
        match action {
            Action::Vacuum => { self.tiles[self.rob] = Tile::Clean; },
            Action::North => { self.rob = self.rob - self.x; },
            Action::South => { self.rob = self.rob + self.x; },
            Action::East => { self.rob = self.rob + 1; },
            Action::West => { self.rob = self.rob - 1; },
        };

        self
    }

    /// Finds a non-optimal path to goal state using depth first search.
    fn depth_first(world: World, arr: Vec<Action>) -> Vec<Action> {
        let new_world = world.do_action(Action::East);
        new_world.actions().collect()
    }

}
