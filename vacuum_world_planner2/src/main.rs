mod world;
use world::Action;
use world::World;
use std::io;
use std::io::BufRead;

use std::collections::BinaryHeap;
use std::collections::HashSet;

#[derive(PartialEq)]
enum Algorithms {
    UniformCost,
    DepthFirst,
    DepthFirstDeepening,
    AStar,
}

static mut nodes_generated: u32 = 0;
static mut nodes_expanded: u32 = 0;

// Read world spec from stdin
fn read_world() -> World {
    let mut line = String::new();
    let stdin = io::stdin();

    stdin.lock().read_line(&mut line).ok().expect("Could not read stdin");
    line.pop(); // Remove new line
    let columns = line.parse::<usize>().ok().expect("Invalid columns");

    line.clear();
    stdin.lock().read_line(&mut line).ok().expect("Could not read stdin");
    line.pop();
    let rows = line.parse::<usize>().ok().expect("Invalid row");

    let mut lines = Vec::new();
    for _ in 0..rows {
        line.clear();
        stdin.lock().read_line(&mut line).ok().expect("Could not read stdin");
        lines.push(line.clone());
    }

    let mut world = world::World::new(rows, columns);
    world.gen_tiles(lines);

    world
}

/// Returns the final world state
fn recurse_ucs(open: &mut BinaryHeap<World>,
               closed: &mut HashSet<World>) -> Option<World> {
    if open.is_empty() {
        println!("open is empty.");
        return None;
    }

    let mut world = open.pop().unwrap();
    if let Some(a) = world.prev_action {
        println!("Performed {:?} into: ", a);
    }
    else {
        println!("Recursed into: ");
    }
    world.print();

    if closed.contains(&world) {
        println!("Dup found!");
        return None;
    }

    if world.is_solved() {
        return Some(world.clone());
    }


    let actions = world.expand();
    for action in actions {
        let mut new_world = world.gen_from(action).unwrap();
        open.push(new_world);
    }

    closed.insert(world);

    while !open.is_empty() {
        let res = recurse_ucs(open, closed);
        if res.is_some() {
            return res;
        }
    }

    None
}

fn backtrack(world: &mut World, mut prevs: &mut HashSet<World>, solution: &mut Vec<Action>) {
    // Unfortunately HashSet doesn't have a get command for keys
    let mut view = world;
    //let mut prevs_vec: Vec<&World> = prevs.iter().collect();
    while view.prev_action.is_some() {
        let action = view.prev_action.unwrap();
        solution.push(action);
        view.undo_action(action);

        for node in prevs.iter() {
            if node == view {
                view.prev_action = node.prev_action;
                break;
            }
        }
        view.print();
    }
    println!("Prev world not found! This shouldn't have happened.");
}

fn uniform_cost(mut world: World) -> Option<Vec<Action>> {
    let mut open = BinaryHeap::new();
    open.push(world);
    let mut closed = HashSet::new();
    let mut solution = Vec::new();
    match recurse_ucs(&mut open, &mut closed) {
        Some(mut w) => {
            //backtrack(&mut w, &mut closed, &mut solution);
            Some(solution)
        },
        None => None,
    }
}

/// Returns true if duplicate is found
fn cycle_check(world: &World, solution: &mut Vec<Action>) -> bool {
    let mut past = world.clone();
    for action in solution.iter().rev() {
        past.undo_action(*action);

        if *world == past {
            return true;
        }
    }

    false
}

/// Modifies the solution vector on the way up
fn recurse_dfs(world: &mut World, solution: &mut Vec<Action>, lim: u32) -> bool {
    if lim == 0 {
        return false;
    }

    if world.is_solved() {
        return true;
    }

    if cycle_check(world, solution) {
        return false;
    }

    unsafe { nodes_expanded += 1; }
    let actions = world.expand();

    for action in actions {
        unsafe { nodes_generated += 1; }

        solution.push(action);
        world.do_action(action);
        if recurse_dfs(world, solution, lim - 1) {
            return true;
        }

        world.undo_action(action);
        solution.pop();
    }
    false
}

fn depth_first(mut world: World) -> Option<Vec<Action>> {
    let mut solution = Vec::new();
    match recurse_dfs(&mut world, &mut solution, u32::max_value()) {
        true => Some(solution),
        false => None,
    }
}

fn depth_first_deepening(mut world: World) -> Option<Vec<Action>> {
    let mut solution = Vec::new();
    let mut block = 1;
    while !recurse_dfs(&mut world, &mut solution, block) {
        solution.clear();
        block += 1;
    }

    Some(solution)
}

fn usage() {
    println!("Usage: ./a ALGO [HEURISTIC]
                Available algorithms are: uniform-cost, depth-first, depth-first-id, a-star
                Available heuristics are: h0, h1, h2");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        usage();
        return;
    }
    let algorithm = match args[1].as_ref() {
        "uniform-cost" => Algorithms::UniformCost,
        "depth-first" => Algorithms::DepthFirst,
        "depth-first-id" => Algorithms::DepthFirstDeepening,
        "a-star" => Algorithms::AStar,
        _ => { usage(); return },
    };

    let heuristic;
    if args.len() == 3 {
        if algorithm != Algorithms::AStar {
            usage();
            return;
        }
        else {
            heuristic = match args[2].as_ref() {
                "h0" => 0,
                "h1" => 1,
                "h2" => 2,
                _ => { usage(); return },
            }
        }
    }


    let world = read_world();

    let solution = match algorithm {
        Algorithms::UniformCost => uniform_cost(world),
        Algorithms::DepthFirst => depth_first(world),
        Algorithms::DepthFirstDeepening => depth_first_deepening(world),
        Algorithms::AStar => None,
    };

    match solution {
        Some(actions) => for a in actions {
            match a {
                Action::Vacuum => println!("V"),
                Action::West => println!("W"),
                Action::South => println!("S"),
                Action::East => println!("E"),
                Action::North => println!("N"),
            }
        },
        None => println!("No solution found."),
    }
    unsafe {
        println!("{} nodes generated", nodes_generated);
        println!("{} nodes expanded", nodes_expanded);
    }
}
