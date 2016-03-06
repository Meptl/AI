use std::hash::Hash;
use std::hash::Hasher;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Copy, Clone, Hash)]
pub enum TileState {
    Clean,
    Dirty,
    Blocked,
}

const NUM_ACTION: usize = 5;
#[derive(Debug, Copy, Clone)]
pub enum Action {
    Vacuum,
    West,
    South,
    East,
    North,
}

#[derive(PartialEq, Copy, Clone, Hash)]
pub struct Tile {
    state: TileState,
    g: u32,
}

impl Tile {
    pub fn new(state: TileState, cost: u32) -> Tile {
        Tile { state:state, g: cost }
    }

    pub fn set_state(&mut self, state: TileState) {
        self.state = state;
    }
}

pub struct World {
    tiles: Vec<Vec<Tile>>,
    rows: usize,
    columns: usize,
    total_dirt: u32,
    vac_row: usize,
    vac_col: usize,
    g: u32,
    h: u32,
    pub prev_action: Option<Action>,
    pub prev_world: Option<*const World>,
}

impl World {
    pub fn new(r: usize, c: usize) -> World {
        World {
            tiles: Vec::with_capacity(r),
            rows: r,
            columns: c,
            total_dirt: 0,
            vac_row: 0,
            vac_col: 0,
            g: 0,
            h: 0,
            prev_action: None,
            prev_world: None,
        }
    }
    /// Receives a list of strings, each string represents a row, each
    /// character represents a column, Input is not checked.
    pub fn gen_tiles(&mut self, lines: Vec<String>) {
        let mut dirts = 0;
        let mut row = 0;
        for line in lines {
            let mut col = 0;
            let mut row_tiles = Vec::with_capacity(self.columns);

            for c in line.chars() {
                match c {
                    '_' => row_tiles.push(Tile::new(TileState::Clean, 1)),
                    '*' => {
                        row_tiles.push(Tile::new(TileState::Dirty, 1));
                        dirts += 1
                    }
                    '#' => row_tiles.push(Tile::new(TileState::Blocked, 1)),
                    '@' => {
                        row_tiles.push(Tile::new(TileState::Clean, 1));
                        self.vac_row = row;
                        self.vac_col = col
                    },
                    _ => continue, // Ignoring some new lines here.
                }
                col += 1;
            }
            self.tiles.push(row_tiles);
            row += 1;
        }
        self.total_dirt = dirts;
    }

    pub fn gen_from(&self, a: Action) -> Option<World> {
        let mut new_node = self.clone();
        // This clone below is really really bad.
        //new_node.prev_world = Some(Box::new(self.clone()));
        new_node.prev_world = Some(self);
        if new_node.do_action(a) {
            return Some(new_node);
        }

        None
    }

    /// The order of branch pushes here determines vacuum priority
    pub fn expand(&mut self) -> Vec<Action> {
        let mut can_actions = Vec::with_capacity(NUM_ACTION);
        let save_action = self.prev_action;

        if self.do_action(Action::Vacuum) {
            can_actions.push(Action::Vacuum);
            self.undo_action(Action::Vacuum);
        }
        if self.do_action(Action::West) {
            can_actions.push(Action::West);
            self.undo_action(Action::West);
        }
        if self.do_action(Action::South) {
            can_actions.push(Action::South);
            self.undo_action(Action::South);
        }
        if self.do_action(Action::East) {
            can_actions.push(Action::East);
            self.undo_action(Action::East);
        }
        if self.do_action(Action::North) {
            can_actions.push(Action::North);
            self.undo_action(Action::North);
        }

        self.prev_action = save_action;
        can_actions
    }

    pub fn do_action(&mut self, a: Action) -> bool {
        let r = self.vac_row;
        let c = self.vac_col;

        self.g += 1; // Each action price is one.
        self.prev_action = Some(a);
        match a {
            Action::Vacuum => {
                if self.get_tile_state(r, c) == TileState::Dirty {
                    self.set_tile_state(r, c, TileState::Clean);
                    self.total_dirt -= 1;
                    return true
                }
            },
            Action::North => {
                if r != 0 &&
                   self.get_tile_state(r - 1, c) != TileState::Blocked {
                    self.vac_row = r - 1;
                    return true
                }
            },
            Action::South => {
                if r != self.rows - 1 &&
                   self.get_tile_state(r + 1, c) != TileState::Blocked {
                    self.vac_row = r + 1;
                    return true
                }
            },
            Action::East => {
                if c != self.columns - 1 &&
                   self.get_tile_state(r, c + 1) != TileState::Blocked {
                    self.vac_col = c + 1;
                    return true
                }
            },
            Action::West => {
                if c != 0 &&
                   self.get_tile_state(r, c - 1) != TileState::Blocked {
                    self.vac_col = c - 1;
                    return true
                }
            },
        }

        self.g -= 1;
        false
    }

    pub fn undo_action(&mut self, a: Action) -> bool {
        let r = self.vac_row;
        let c = self.vac_col;

        self.g += 1; // Each action price is one
        match a {
            Action::Vacuum => {
                if self.get_tile_state(r, c) == TileState::Clean {
                    self.set_tile_state(r, c, TileState::Dirty);
                    self.total_dirt += 1;
                    return true
                }
            },
            Action::North => return self.do_action(Action::South),
            Action::South => return self.do_action(Action::North),
            Action::East => return self.do_action(Action::West),
            Action::West => return self.do_action(Action::East),
        }

        self.g -= 1;
        false
    }

    pub fn test_undo(&mut self, a: Action) -> bool {
        let r = self.vac_row;
        let c = self.vac_col;

        self.g += 1; // Each action price is one
        match a {
            Action::Vacuum => {
                if self.get_tile_state(r, c) == TileState::Clean {
                    self.set_tile_state(r, c, TileState::Dirty);
                    self.total_dirt += 1;
                    return true
                }
            },
            Action::South => {
                if r != 0 &&
                   self.get_tile_state(r - 1, c) != TileState::Blocked {
                    self.vac_row = r - 1;
                    return true
                }
            },
            Action::North => {
                if r != self.rows - 1 &&
                   self.get_tile_state(r + 1, c) != TileState::Blocked {
                    self.vac_row = r + 1;
                    return true
                }
            },
            Action::West => {
                if c != self.columns - 1 &&
                   self.get_tile_state(r, c + 1) != TileState::Blocked {
                    self.vac_col = c + 1;
                    return true
                }
            },
            Action::East => {
                if c != 0 &&
                   self.get_tile_state(r, c - 1) != TileState::Blocked {
                    self.vac_col = c - 1;
                    return true
                }
            },
        }

        self.g -= 1;
        false
    }

    fn set_tile_state(&mut self, r: usize, c: usize, s: TileState) {
        self.tiles.get_mut(r).unwrap()
                  .get_mut(c).unwrap()
                  .set_state(s);
    }

    fn get_tile_state(&self, r: usize, c: usize) -> TileState {
        self.tiles.get(r).unwrap().get(c).unwrap().state
    }

    pub fn is_solved(&self) -> bool {
        self.total_dirt == 0
    }

    pub fn print(&self) {
        println!("Vacuum @ row: {}, col: {}, dirt: {}",
                 self.vac_row, self.vac_col, self.total_dirt);
        for t in self.tiles.iter() {
            for x in t {
                match x.state {
                    TileState::Clean => print!("_"),
                    TileState::Dirty => print!("*"),
                    TileState::Blocked => print!("#"),
                }
            }
            println!("");
        }
    }
}

impl Clone for World {
    fn clone(&self) -> Self {
        let mut new_tiles = Vec::with_capacity(self.rows);
        for row in self.tiles.iter() {
            new_tiles.push((*row).clone());
        }

        World {
            tiles: new_tiles,
            rows: self.rows,
            columns: self.columns,
            total_dirt: self.total_dirt,
            vac_row: self.vac_row,
            vac_col: self.vac_col,
            g: self.g,
            h: self.h,
            prev_action: self.prev_action.clone(),
            prev_world: self.prev_world,
        }
    }
}

impl Eq for World {
}

impl PartialEq for World {
    fn eq(&self, other: &World) -> bool {
        self.vac_row == other.vac_row &&
        self.vac_col == other.vac_col &&
        self.total_dirt == other.total_dirt
    }
}


impl Ord for World {
    fn cmp(&self, other: &World) -> Ordering {
        let my_f = self.g + self.h;
        let other_f = other.g + other.h;
        other_f.cmp(&my_f)
    }
}

impl PartialOrd for World {
    fn partial_cmp(&self, other: &World) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for World {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        let ref x = self.tiles;
        for row in x {
            for tile in row {
                tile.hash(state);
            }
        }
    }
}
