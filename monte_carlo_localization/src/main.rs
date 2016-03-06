extern crate rand;

mod monte_carlo;

use monte_carlo::{Action, Particles};
use std::io::{BufRead, Write};

const DEBUG: bool = true;
const DEFAULT_SIGMA: f64 = 2.0;
const DEFAULT_PARTICLES: usize = 100;

/// Print to stderr
macro_rules! debug {
    ($($arg:tt)*) => (
        if DEBUG {
            match writeln!(&mut ::std::io::stdout(), $($arg)*) {
                Ok(_) => {},
                Err(x) => panic!("Unable to write to stderr: {}", x),
            }
        }
    )
}

/// Retrieves Optional sigma and particle number. Returns defaults
fn read_args() -> (f64, usize) {
    let mut sigma = DEFAULT_SIGMA;
    let mut particles = DEFAULT_PARTICLES;
    let mut args = std::env::args();
    loop {
        match args.next() {
            Some(arg) => {
                match arg.as_ref() {
                    "--sigma" => {
                        sigma = args.next().unwrap()
                                    .parse::<f64>().ok()
                                    .expect("--sigma expects float")
                    },
                    "--particles" => {
                        particles = args.next().unwrap()
                                        .parse::<usize>().ok()
                                        .expect("--particles expects int")
                    },
                    _ => {}
                }
            },
            None => break,
        }
    }
    (sigma, particles)
}

fn read_description() -> Vec<Vec<bool>> {
    let stdin = std::io::stdin();
    let mut line_iter = stdin.lock().lines().filter(|line| {
        match line.as_ref() {
            Ok(val) => !val.is_empty() && !val.starts_with("/"),
            Err(e) => panic!("{}", e),
        }
    }).map(|line| line.unwrap());

    let col_count = line_iter.next().unwrap().parse::<usize>().unwrap();
    let row_count = line_iter.next().unwrap().parse::<usize>().unwrap();

    // Grid will be indexed Y first then X with <0,0> being the "bottom left"
    // which will be the final input.
    let grid: Vec<Vec<bool>> = line_iter.take(row_count).map(|line| {
        line.chars().filter_map(|c| {
            match c {
                '_' => Some(false),
                '#' => Some(true),
                ' ' => None,
                e => panic!("Unknown grid tile {}", e),
            }
        }).collect()
    }).collect();

    assert!(grid.iter().all(|row| row.len() == col_count), "Invalid row data");
    // Reverse final grid output.
    grid.iter().rev().cloned().collect()
}

fn read_sensor() -> Option<(f64, f64, Action)> {
    let stdin = std::io::stdin();
    let mut line_iter = stdin.lock().lines().map(|line| line.unwrap());

    let line = line_iter.next().unwrap();
    if line == "END" {
        return None;
    }

    let mut word_iter = line.split_whitespace();
    let x = word_iter.next().expect("No END").parse::<f64>().unwrap();
    let y = word_iter.next().unwrap().parse::<f64>().unwrap();
    let action = match word_iter.next().unwrap() {
        "EAST" => Action::East,
        "WEST" => Action::West,
        "NORTH" => Action::North,
        "SOUTH" => Action::South,
        _ => return None
    };

    Some((x, y, action))
}

fn main() {
    let (sigma, particle_cnt) = read_args();
    let variance = sigma.powi(2);

    let map = read_description();
    let mut rng = rand::thread_rng();
    debug!("Sigma: {}, Particles: {}", sigma, particle_cnt);

    let mut particles = Particles::new(particle_cnt, map, &mut rng);
    loop {
        match read_sensor() {
            Some((x, y, act)) => {
                particles.perform_action(act);
                particles.update_weights(x, y, variance);
                particles.resample();
                particles.print();
            },
            None => {
                break;
            },
        }
    }
}
