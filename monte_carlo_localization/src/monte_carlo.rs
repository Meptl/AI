use ::rand::ThreadRng;
use ::rand::distributions::{IndependentSample, Range};

#[derive(Copy, Clone)]
pub enum Action {
    North,
    South,
    East,
    West
}

///  A singular particle
#[derive(Debug, Clone)]
struct Particle {
    pub x: usize,
    pub y: usize,
    pub weight: f64,
}

impl Particle {
    pub fn new_rand(map: &Vec<Vec<bool>>, mut rng: &mut ThreadRng) -> Particle {
        let xrange = Range::new(0, map[0].len());
        let yrange = Range::new(0, map.len());
        let mut x = xrange.ind_sample(rng);
        let mut y = yrange.ind_sample(rng);

        // Ensure tile is not blocked.
        while map[y][x] {
            x = xrange.ind_sample(rng);
            y = yrange.ind_sample(rng);
        }

        Particle {
            x: x,
            y: y,
            weight: 0.0,
        }
    }

    pub fn perform_action(&mut self,
                          act: Action,
                          map: &Vec<Vec<bool>>,
                          mut rng: &mut ThreadRng) {
        let range = Range::new(0.0, 1.0);
        let chance = range.ind_sample(rng);
        let mut action = act;

        if chance < 0.7 {
            ;
        } else if chance < 0.8 {
            match action {
                Action::North => action = Action::West,
                Action::South => action = Action::East,
                Action::East => action = Action::North,
                Action::West => action = Action::South,
            }
        } else if chance < 0.9 {
            match action {
                Action::North => action = Action::East,
                Action::South => action = Action::West,
                Action::East => action = Action::South,
                Action::West => action = Action::North,
            }
        } else {
            return;
        }

        let (goalx, goaly) =
            match action {
                Action::North => (self.x, self.y.wrapping_add(1)),
                Action::South => (self.x, self.y.wrapping_sub(1)),
                Action::East => (self.x.wrapping_add(1), self.y),
                Action::West => (self.x.wrapping_sub(1), self.y),
            };

        if goalx >= map.len() || goaly >= map.len() || map[goaly][goalx] {
            return;
        }
        self.x = goalx;
        self.y = goaly;
    }

    pub fn update_weight(&mut self, sensor_x: f64,
                          sensor_y: f64, variance: f64) {
        let (my_x, my_y) = (self.x as f64, self.y as f64);
        let numerator = (my_x - sensor_x).powi(2) + (my_y - sensor_y).powi(2);
        self.weight =  (numerator / (-2. * variance)).exp();
    }
}

/// Represents a group of particles on a map
pub struct Particles<'a> {
    nodes: Vec<Particle>,
    map: Vec<Vec<bool>>,
    rng: &'a mut ThreadRng,
}

impl<'a> Particles<'a> {
    pub fn new(particle_cnt: usize,
               map: Vec<Vec<bool>>,
               mut rng: &mut ThreadRng) -> Particles {
        Particles {
            nodes: (0..particle_cnt).map(|_| {
                Particle::new_rand(&map, &mut rng)
            }).collect(),
            map: map,
            rng: rng
        }
    }

    /// Perform an action for all nodes.
    pub fn perform_action(&mut self, act: Action) {
        for node in self.nodes.iter_mut() {
            node.perform_action(act, &self.map, &mut self.rng);
        }
    }

    /// Change particle weights based on sensor data
    pub fn update_weights(&mut self, x: f64, y: f64, variance: f64) {
        for node in self.nodes.iter_mut() {
            node.update_weight(x, y, variance);
        }
    }

    /// Sample from the current nodes
    pub fn resample(&mut self) {
        let total_weight = self.nodes.iter().fold(0., |accum, item| accum + item.weight);
        let range = Range::new(0., total_weight);

        let mut new_nodes = Vec::new();
        let part_cnt = (self.nodes.len() as f64 * 0.9) as usize;
        for _ in 0..part_cnt {
            let pick = range.ind_sample(self.rng);

            let mut cumulative_weight = 0.;
            for node in self.nodes.iter() {
                cumulative_weight += node.weight;
                if cumulative_weight > pick {
                    new_nodes.push(node.clone());
                    break;
                }
            }
        }

        // 10% of particles are uniformly sampled.
        for _ in new_nodes.len()..self.nodes.len() {
            new_nodes.push(Particle::new_rand(&self.map, &mut self.rng));
        }

        self.nodes = new_nodes;
    }

    pub fn print(&self) {
        for node in self.nodes.iter() {
            println!("{} {} {}", node.x, node.y, node.weight);
        }
    }
}
