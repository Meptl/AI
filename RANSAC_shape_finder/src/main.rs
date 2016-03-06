#![allow(unused_imports)]
extern crate rand;

mod ransac;

use ransac::{Point, Line, Ransac};
use rand::{ThreadRng};
use rand::distributions::{IndependentSample, Range};
use std::io::{BufRead, Write};


const MIN_INLIER: usize = 5;
const MAX_INLIER_DIST: usize = 2;
const MAX_GAP_SIZE: usize = 4;
const SEARCH_LIMIT: usize = 500;

/// Print to stderr
macro_rules! debug {
    ($($arg:tt)*) => (
        /*
        match writeln!(&mut ::std::io::stdout(), $($arg)*) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
        */
    )
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
    // which will be the final input. We have to jump through a few hoops to
    // get the information sorted.
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

fn filter_points(grid: &Vec<Vec<bool>>) -> Vec<Point> {
    grid.iter().enumerate().fold(Vec::new(), |accum, (y, row)| {
        let mut new_points = accum.clone();
        new_points.extend(row.iter().enumerate().filter_map(|(x, &tile)| {
            match tile {
                true => Some(Point::new(x as f64, y as f64)),
                false => None,
            }
        }));
        new_points
    })
}

/// Generates a line between 2 random points, returning slope and intercept
fn random_line(points: &Vec<Point>, rng: &mut ThreadRng) -> Line {
    let range = Range::new(0, points.len());
    let point1 = unsafe { points.get_unchecked(range.ind_sample(rng)) };
    let point2 = unsafe { points.get_unchecked(range.ind_sample(rng)) };
    debug!("{:?}, {:?}", point1, point2);
    Ransac::line(point1, point2)
}

fn find_best_line<'a>(init_model: &Line,
                  points: &'a Vec<Point>,
                  inliers: &Vec<&'a Point>) -> (Line, Vec<&'a Point>) {
    let mut old_model = init_model.clone();
    let mut old_inliers = inliers.clone();

    loop {
        let new_model = Ransac::simple_linear_regression(&old_inliers);
        let new_inliers: Vec<&Point> = Ransac::find_inliers(&new_model,
                                                            points,
                                                            MAX_INLIER_DIST);
        let new_segment_inliers = Ransac::find_linesegment(&new_model,
                                                           &new_inliers,
                                                           MAX_GAP_SIZE);
        debug!("new line {:?} with {} inliers", new_model, new_segment_inliers.len());
        if new_segment_inliers.len() <= old_inliers.len() {
            break;
        }
        else {
            old_model = new_model.clone();
            old_inliers = new_segment_inliers.clone();
        }
    }
    (old_model, old_inliers)
}

/// Modifies obstacles with Line segments
fn ransac(count: usize, obstacles: &mut Vec<(Point, Point)>,
          points: Vec<Point>, mut rng: ThreadRng) {
    if count >= SEARCH_LIMIT || points.len() < MIN_INLIER {
        return;
    }

    let init_model = random_line(&points, &mut rng);
    let init_inliers: Vec<&Point> = Ransac::find_inliers(&init_model,
                                                         &points,
                                                         MAX_INLIER_DIST);
    let inliers = Ransac::find_linesegment(&init_model,
                                           &init_inliers,
                                           MAX_GAP_SIZE);
    debug!("init line {:?} with {} inliers", init_model, inliers.len());

    if inliers.len() <= MIN_INLIER {
        return ransac(count + 1, obstacles, points.clone(), rng);
    }

    let (fitted_model, most_inliers) = find_best_line(&init_model,
                                                      &points,
                                                      &inliers);
    debug!("new line {:?} with {} inliers", fitted_model, most_inliers.len());
    let left_x = most_inliers.get(0).unwrap().x;
    let right_x = most_inliers.get(most_inliers.len() - 1).unwrap().x;
    let start_point = Point::new(left_x, fitted_model.eval(left_x));
    let end_point = Point::new(right_x, fitted_model.eval(right_x));
    obstacles.push((start_point, end_point));

    let new_points: Vec<Point> =
        points.iter().filter(|x| {
            !most_inliers.iter().any(|&val| val as *const Point == *x as *const Point)
        }).cloned().collect();
    debug!("{} -> {}", points.len(), new_points.len());
    return ransac(0, obstacles, new_points, rng);
}

fn main() {
    let points = filter_points(&read_description());
    let rng = rand::thread_rng();
    let mut obstacles = Vec::new();
    ransac(0, &mut obstacles, points, rng);

    println!("number of circles: 0");
    println!("number of lines: {}", obstacles.len());
    for (start, end) in obstacles {
        println!("{} {} {} {}", start.x, start.y, end.x, end.y);
    }
}
