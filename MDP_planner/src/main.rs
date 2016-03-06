#![allow(unused_imports, unused_variables)]
mod mdp;

use mdp::{Action, State, Transition};
use std::io::{BufRead, Write};

/// This program assumes the mdp given in stdin is in proper format.

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

#[derive(Debug)]
enum Alg {
    ValueIteration,
}

fn read_args() -> (Alg, f64, f64) {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        println!("Usage: cue6_09 [ALG] [DISCOUNT_FACTOR] \
                                 [TERMINATION CRITERION]");
    }

    let alg = match args.get(1).unwrap().as_ref() {
        "vi" => Alg::ValueIteration,
        _ => panic!("Available algorithms: vi"),
    };

    let discount = args.get(2).unwrap().parse::<f64>().unwrap();

    let term_criterion = args.get(3).unwrap().parse::<f64>().unwrap();

    debug!("{:?} alg, discount {}, terminate {}", alg, discount, term_criterion);

    (alg, discount, term_criterion)
}

fn read_description() -> Vec<State> {
    let stdin = std::io::stdin();
    let mut line_iter = stdin.lock().lines().filter(|line| {
        match line.as_ref() {
            Ok(val) => !val.is_empty() && !val.starts_with("#"),
            Err(e) => panic!("{}", e),
        }
    }).map(|line| line.unwrap());

    let num_state_line = line_iter.next().unwrap();
    let num_states = num_state_line.split_whitespace()
                                   .last().unwrap()
                                   .parse::<usize>().unwrap();

    let start_state_line = line_iter.next().unwrap();
    let start_state = start_state_line.split_whitespace()
                                       .last().unwrap()
                                       .parse::<usize>().unwrap();

    debug!("{} states, starting at {}", num_states, start_state);

    let mut states = Vec::with_capacity(num_states);
    for i in 0..num_states {
        let (reward, term, act) = read_state_desc(&line_iter.next().unwrap());
        let mut actions = Vec::with_capacity(act);
        for i in 0..act {
            let mut new_action = read_action(&line_iter.next().unwrap());
            new_action.set_id(i);
            actions.push(new_action);
        }
        states.push(State::new(i, reward, term, actions));
    }

    states
}

fn read_state_desc(line: &str) -> (f64, bool, usize) {
    let mut word_iter = line.split_whitespace();
    let reward = word_iter.next().unwrap().parse::<f64>().unwrap();
    let terminal = match word_iter.next().unwrap().parse::<u8>().unwrap() {
        1 => true,
        0 => false,
        _ => panic!("Improper terminal description"),
    };
    let actions = word_iter.next().unwrap().parse::<usize>().unwrap();

    (reward, terminal, actions)
}

fn read_action(line: &str) -> Action {
    let mut tokens = line.split_whitespace();
    let succ_num = tokens.next().unwrap().parse::<usize>().unwrap();

    let mut trans = Vec::with_capacity(succ_num);
    for _ in 0..succ_num {
        let dest = tokens.next().unwrap().parse::<usize>().unwrap();
        let prob = tokens.next().unwrap().parse::<f64>().unwrap();

        trans.push(Transition::new(dest, prob));
    }

    Action::new(trans)
}

fn value_iteration(prev_util: Vec<f64>,
                   states: Vec<State>,
                   discount_factor: f64,
                   termination_criterion: f64) {
    debug!("{:?}", prev_util);
    let new_util: Vec<f64> =
        states.iter().map(|state| {
            state.value_iterate(&prev_util, discount_factor)
        }).collect();


    let terminate = !new_util.iter().zip(prev_util.iter())
               .any(|(new, prev)| {
                   debug!("delta {}", f64::abs(new - prev));
                   f64::abs(new - prev) > termination_criterion
               });

    match terminate {
        false => value_iteration(new_util, states,
                                 discount_factor, termination_criterion),
        true => {
            for state in states {
                match state.best_action(&new_util) {
                    Some(action) => println!("{}", action.get_id()),
                    None => println!(""),
                }
            }
            unsafe {println!("{} backups performed", mdp::BACKUPS); }
        }
    }
}

fn main() {
    let (alg, dis, term) = read_args();
    let states = read_description();
    let init_util = std::iter::repeat(0_f64).take(states.len()).collect();

    match alg {
        vi => value_iteration(init_util, states, dis, term),
    };
}
