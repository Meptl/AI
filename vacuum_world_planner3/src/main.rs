/// Outputs a movement plan for a world description given in stdin.
/// See the README for world formatting.

mod world;
use std::io::BufRead;
use world::{Tile, World};

const USAGE_DESCRIPTION: &'static str =
    "vacuum-world-planner3 ALG [OPTIONS]
         ALG is one of:
             --depth-first
             --depth-first-id
             --breadth-first
             --a-star HEURISTIC
                 HEURISTIC is one of h0, h1, h2
         -h --help
         -v --visual
         ";

enum Algorithm {
    DepthFirst,
    DepthFirstIteratedDeepening,
    BreadthFirst,
    AStar
}

struct ProgramOptions {
    algorithm: Algorithm,
    heuristic: usize,
    visual: bool,
}

fn usage() {
    println!("{}", USAGE_DESCRIPTION);
}

/// Reads program arguments for algorithm type
fn read_args() -> Option<ProgramOptions> {
    let mut args = std::env::args().skip(1);
    let mut opts = ProgramOptions {
        algorithm: Algorithm::DepthFirst,
        heuristic: 0,
        visual: false,
    };

    let mut alg = None;

    loop {
        let arg = match args.next() {
            Some(arg) => arg,
            None => break,
        };

        match arg.as_ref() {
            "--depth-first" => alg = Some(Algorithm::DepthFirst),
            "--depth-first-id" =>
                alg = Some(Algorithm::DepthFirstIteratedDeepening),
            "--breadth-first" => alg = Some(Algorithm::BreadthFirst),
            "--a-star" => {
                // Get next argument - heuristic value
                let heu = match args.next() {
                    Some(arg) => arg,
                    None => return None,
                };
                match heu.as_ref() {
                    "h0" => opts.heuristic = 0,
                    "h1" => opts.heuristic = 1,
                    "h2" => opts.heuristic = 2,
                    _ => return None,
                }
                alg = Some(Algorithm::AStar)
            },
            "--help" => return None,
            "-h" => return None,
            "--visual" => opts.visual = true,
            "-v" => opts.visual = true,
            x => {
                println!("Unknown argument: {}", x);
                return None;
            }
        }
    }

    if alg.is_none() {
        return None;
    } else {
        opts.algorithm = alg.unwrap();
    }

    Some(opts)
}

/// Receives a line of the world_spec. Adds each value into the world
/// description.
///
/// Ok contains a set of tiles and a robot location if one
/// was found. Otherwise returns an Err describing the error.
fn parse_world_line(line: &str, x: usize)
    -> Result<(Vec<Tile>, Option<usize>), String> {
    let mut tile_col = 0;
    let mut rob = None;
    let mut world_desc = Vec::new();

    for tile_char in line.chars() {
        world_desc.push(match tile_char {
            '#' => Tile::Blocked,
            '_' => Tile::Clean,
            '*' => Tile::Dirty,
            '@' => {
                if rob.is_some() {
                    return Err(String::from("Found two robots."));
                } else {
                    rob = Some(tile_col);
                }
                Tile::Clean
            },
            x => return Err(format!("Unknown world tile {}", x)),
        });

        tile_col += 1;
    }

    if world_desc.len() != x {
        return Err(format!("Found {} columns, expected {}",
                           world_desc.len(), x));
    }

    Ok((world_desc, rob))
}

/// Reads world data from stdin
fn read_world() -> Option<World> {
    let stdin = std::io::stdin();
    let mut line_iter = stdin.lock().lines().map(|x| x.unwrap());

    // Read data size
    let x_in = line_iter.next().and_then(|x| x.parse::<usize>().ok());
    let y_in = line_iter.next().and_then(|y| y.parse::<usize>().ok());
    let (x_len, y_len) = match (x_in, y_in) {
        (Some(x), Some(y)) => (x, y),
        _ => {
            println!("Error parsing row and column length");
            return None;
        },
    };

    let mut world_desc = Vec::new();
    let mut robot_pos = None;

    // Read world description
    for row in 0..y_len {
        let line = match line_iter.next() {
            Some(l) => l,
            None => {
                println!("Unexpected end of world spec.");
                return None;
            }
        };

        match parse_world_line(&line, x_len) {
            Ok((mut world_line, None)) => world_desc.append(&mut world_line),
            Ok((mut world_line, Some(rob))) => {
                match robot_pos {
                    Some(_) => {
                        println!("Error in world specification: \
                                  Found two robots.");
                        return None;
                    },
                    None => robot_pos = Some(row * x_len + rob),
                }
                world_desc.append(&mut world_line)
            }
            Err(e) => {
                println!("Error in world specification: {}", e);
                return None;
            }
        }
    }

    if robot_pos.is_none() {
        println!("No robot found in world.");
        return None;
    }

    Some(World::new(world_desc, x_len, y_len, robot_pos.unwrap()))
}

fn main() {
    let opts = read_args();
    if opts.is_none() {
        usage();
        return;
    }

    let world = read_world();
}
