#![allow(unused_imports)]
extern crate rand;
use rand::Rng;

mod world;
use world::World;
use world::Robot;
use world::euclid_dist;

use std::io;
use std::io::Stdin;
use std::io::BufRead;
use std::str::FromStr;

/// Read line and parse a type from it, removing newlines
fn read_stdin<T>(stdin: &Stdin, line: &mut String) -> T where T: FromStr {
    line.clear();
    stdin.lock().read_line(line).ok().expect("Could not read stdin");
    line.pop();
    line.parse::<T>().ok().expect("Invalid usize input")
}

/// Reads world spec from stdin and generates it
fn read_world<'a>() -> (World, Robot<'a>) {
    let mut line = String::new();
    let stdin = io::stdin();

    let total_columns = read_stdin::<usize>(&stdin, &mut line);
    let total_rows = read_stdin::<usize>(&stdin, &mut line);

    let mut tile_data = Vec::new();
    for _ in 0..total_rows {
        let mut line_vec = Vec::new();
        line.clear();
        stdin.lock().read_line(&mut line).ok().expect("Could not read stdin");
        for c in line.chars() {
            match c {
                '_' => line_vec.push(0),
                '#' => line_vec.push(1),
                '\n' => continue,
                x => println!("Unknown input: {}", x),
            }
        }
        tile_data.push(line_vec);
    }

    let robot_column = read_stdin::<f64>(&stdin, &mut line);
    let robot_row = read_stdin::<f64>(&stdin, &mut line);
    let goal_column = read_stdin::<f64>(&stdin, &mut line);
    let goal_row = read_stdin::<f64>(&stdin, &mut line);

    let world = World::new(total_rows, total_columns, tile_data);
    let mut robot = Robot::new(robot_row, robot_column);
    robot.set_goal(goal_row, goal_column);

    (world, robot)
}

fn main() {
    let (world, mut robot) = read_world();
    // Each robot has its own goal, but we are only concerned with one
    let my_goal = robot.goal.unwrap();
    robot.set_world(&world);

    let blank_rob = Robot::new(my_goal.0, my_goal.1);
    let mut robot_tree = Vec::new();
    let mut rng = rand::thread_rng();

    robot_tree.push(robot);
    loop {
        let bias: f64 = rng.gen();
        let dest = if bias > 0.05 { world.get_random_point() } else { my_goal };
        let mut new_node;
        {
            let mut closest_node = &blank_rob;
            let mut lowest_distance = std::f64::MAX;
            for node in robot_tree.iter() {
                let dist = euclid_dist(node.pos, dest);
                if lowest_distance > dist {
                    lowest_distance = dist;
                    closest_node = node;
                }
            }
            new_node = closest_node.clone();
        }

        if new_node.move_to(dest.0, dest.1) {
            if new_node.is_done() {
                robot_tree.push(new_node);
                break;
            }
            robot_tree.push(new_node);
        }
    }

    let result = robot_tree.pop().unwrap();
    for i in result.moves {
        println!("{} {}", i.1, i.0);
    }
}
