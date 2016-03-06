#[derive(Debug, Clone)]
pub struct State {
    id: usize,
    reward: f64,
    terminal: bool,
    actions: Vec<Action>,
}

pub static mut BACKUPS: usize = 0;

impl State {
    pub fn new(id: usize,
               reward: f64,
               terminal: bool,
               actions: Vec<Action>) -> State {
        State {
            id: id,
            reward: reward,
            terminal: terminal,
            actions: actions,
        }
    }

    pub fn value_iterate(&self, utilities: &Vec<f64>, discount: f64) -> f64 {
        unsafe { BACKUPS += 1; }
        match self.terminal {
            true => self.reward,
            false => self.reward + discount * self.max_expect(utilities).1
        }
    }

    pub fn best_action(&self, utilities: &Vec<f64>) -> Option<Action> {
        match self.terminal {
            true => None,
            false => self.max_expect(utilities).0
        }
    }

    // Finds the action with the best utility given a util table
    // Returns both the action and its expectation
    fn max_expect(&self, utilities: &Vec<f64>) -> (Option<Action>, f64) {
        let mut max = -10000000_f64; // A arbitrarily low number
        let mut max_act = None;
        for action in self.actions.iter() {
            //println!("{} action {}", self.id, action.get_id());
            let expect = action.expectation(utilities);
            //println!("Got expectation {}", expect);
            if expect > max {
                max = expect;
                max_act = Some(action)
            }
        }

        if max_act.is_none() {
            panic!("No expectation could be made.");
        }
        //println!("Returning max_val {} from action {}" , max, max_act.get_id());
        (Some(max_act.unwrap().clone()), max)
    }
}

#[derive(Debug, Clone)]
pub struct Action {
    id: usize,
    pub transitions: Vec<Transition>
}

impl Action {
    pub fn new(trans: Vec<Transition>) -> Action {
        Action {
            id: 0,
            transitions: trans,
        }
    }

    // Using utility, calculate the weighted expectation of all transitions.
    pub fn expectation(&self, utilities: &Vec<f64>) -> f64 {
        let x = self.transitions.iter().fold(0_f64, |acc, trans| {
            acc + trans.prob() * utilities.get(trans.dest()).unwrap()
        });
        x
    }

    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

#[derive(Debug, Clone)]
pub struct Transition {
    dest: usize,
    prob: f64,
}

impl Transition {
    pub fn new(dest: usize, prob: f64) -> Transition {
        Transition {
            dest: dest,
            prob: prob,
        }
    }

    pub fn dest(&self) -> usize {
        self.dest
    }

    pub fn prob(&self) -> f64 {
        self.prob
    }
}
