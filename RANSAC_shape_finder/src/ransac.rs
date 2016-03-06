use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point {
            x: x,
            y: y
        }
    }
}

#[derive(Debug, Clone)]
pub struct Line {
    pub slope: f64,
    pub intercept: f64,
}

impl Line {
    pub fn new(m: f64, b: f64) -> Line {
        Line {
            slope: m,
            intercept: b,
        }
    }

    pub fn eval(&self, x: f64) -> f64 {
        self.slope * x + self.intercept
    }
}

pub struct Ransac;

/// Some functions work over iterators of Points stored as
/// Box<Iterator<Item=&'a Point> + 'a>;
impl Ransac {
    /// Returns line between two points
    pub fn line(p1: &Point, p2: &Point) -> Line {
        let slope = (p2.y - p1.y) / (p2.x - p1.x);
        let intercept = p2.y - slope * p2.x;
        Line::new(slope, intercept)
    }

    /// Performs simple linear regression to generate a new line
    pub fn simple_linear_regression(points: &Vec<&Point>) -> Line {
        let n = points.len() as f64;
        let (x_sum, y_sum, x2_sum, xy_sum) =
            points.iter().fold((0.0, 0.0, 0.0, 0.0), |accum, pnt| {
                (accum.0 + pnt.x,
                 accum.1 + pnt.y,
                 accum.2 + pnt.x.powi(2),
                 accum.3 + pnt.x * pnt.y)
            });

        let x_y_bar = x_sum * y_sum / n;
        let slope = (xy_sum - x_y_bar) / (x2_sum - (x_sum.powi(2) / n));

        let intercept = (y_sum - slope * x_sum) / n;

        Line::new(slope, intercept)
    }


    /// Finds all points that are a given distance from the line
    pub fn find_inliers<'a>(line: &Line,
                            points: &'a Vec<Point>,
                            dist: usize) -> Vec<&'a Point> {
        points.iter().filter(move |point| {
            let diff = point.y - (line.slope * point.x + line.intercept);
            diff.abs() < (dist as f64)
        }).collect()
    }

    /// Given a line and its inliers determines how many segments there are
    /// and returns the inliers in the largest one.
    pub fn find_linesegment<'a>(line: &Line,
                                inliers: &Vec<&'a Point>,
                                gap_limit: usize) -> Vec<&'a Point> {
        let mut u_values: Vec<(f64, &Point)> =
            inliers.iter().map(|&pnt| {
                let numerator = pnt.x + (pnt.y - line.intercept) * line.slope;
                (numerator / (line.slope.powi(2) + 1.0), pnt)
            }).collect();
        u_values.sort_by(|&(u1, _), &(u2, _)|
             if u1 < u2 { Ordering::Less }
             else if u1 > u2 { Ordering::Greater }
             else { Ordering::Equal });

        // A vector of gap indicies.
        // Will create an array of [0, GAPS, gaps.len()]
        let gaps: Vec<usize> =
            (0..1).chain(u_values.windows(2)
                                 .map(|win| win[1].0 - win[0].0)
                                 .enumerate()
                                 .filter(|&(_, delta)| delta > (gap_limit as f64))
                                 .map(|(i, _)| i + 1))
                  .chain((u_values.len()..u_values.len() + 1))
                  .collect();

        // Find largest gap, ideally would use max_by, but that is unstable.
        let mut max_seg: [usize; 2] = [0, 0];
        let mut max_size = 0;
        for win in gaps.windows(2) {
            if win[1] - win[0] > max_size {
                max_seg = [win[0], win[1]];
                max_size = win[1] - win[0];
            }
        }
        let new_inliers: Vec<&Point> =
            u_values[max_seg[0]..max_seg[1]]
                    .iter().map(|&(_, pnt)| pnt).collect();


        /*
        println!("UVALUES {:?}", u_values);
        println!("SEGMENTS {:?}", gaps);
        println!("LARGEST SEGMENT {:?}", max_seg);
        println!("NEW_INLIERS {:?}", new_inliers);
        */

        new_inliers
    }
}
