extern crate regex;
use regex::Regex;

mod cnf;
use cnf::*;
use cnf::TermType::*;

use std::io::{Write, BufRead};

/// Print to stderr
macro_rules! debug {
    ($($arg:tt)*) => (
        match writeln!(&mut ::std::io::stdout(), $($arg)*) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    )
}

fn read_line() -> Option<String> {
    let stdin = std::io::stdin();
    let mut line = String::new();
    let value = match stdin.lock().read_line(&mut line) {
        Ok(0) => None, // EOF
        Ok(_) => Some(line),
        Err(_) => None
    };
    value
}

fn parse_clause(line: &str) -> Clause {
    debug!("parse_clause: {}", line);
    let re = Regex::new(r"-?[:upper:]\S*([(][^|]*[)])").unwrap();

    let mut clause = Clause::new();
    for cap in re.captures_iter(&line) {
        clause.literals.push(parse_literal(cap.at(0).unwrap()));
    }
    clause
}

fn parse_literal(line: &str) -> Literal {
    debug!("parse_literal: {}", line);
    let neg = line.chars().next().unwrap() == '-';
    if neg {
        Literal::new(parse_predicate(&line[1..]), neg)
    } else {
        Literal::new(parse_predicate(line), neg)
    }
}

fn parse_predicate(line: &str) -> Predicate {
    debug!("parse_predicate: {}", line);
    let index = line.find('(').unwrap();
    let name = String::from(&line[..index]);
    let mut predicate = Predicate::new(name);

    for term in parse_termlist(&line[index + 1..line.len() - 1]) {
        predicate.terms.push(term);
    }
    predicate
}

fn parse_termlist(line: &str) -> Vec<Term> {
    debug!("parse_termlist: {}", line);
    let mut terms = Vec::new();
    let mut prev = 0;
    for (i, c) in line.char_indices() {
        if c == ',' {
            let substr = &line[prev..i];
            if balanced_paren(substr) {
                prev = i + 1;
                terms.push(parse_term(substr));
            }
        }
    }

    // Last term
    terms.push(parse_term(&line[prev..]));
    terms
}

fn parse_term(line: &str) -> Term {
    let trim = line.trim();
    debug!("parse_term: {}", trim);
    match trim.find('(') {
        Some(i) => {
            let mut term = Term::new(String::from(&trim[..i]));
            debug!("Parsing: {}", &trim[i + 1..trim.len()]);
            for t in parse_termlist(&trim[i + 1..trim.len() - 1]) {
                term.terms.push(t);
            }
            term
        }
        None => {
            Term::new(String::from(trim))
        }
    }
}

/// Checks if a line has balanced parentheses by counting '(' and ')'
fn balanced_paren(line: &str) -> bool {
    let mut count = 0;
    for c in line.chars() {
        match c {
            '(' => count = count + 1,
            ')' => count = count - 1,
            _ => continue,
        }
    }

    count == 0
}

/// Performs unification on each line of knowledge base with another
fn unify(kb: Vec<Clause>) {
    let mut kb_iter = kb.iter();
    loop {
        match kb_iter.next() {
            Some(line) => {
                let other_iter = kb_iter.clone();
                for other in other_iter {
                    unify_cc(line, other);
                }
            },
            None => { break },
        }
    }
}

/// Performs unification on each predicate in focus on each predicate in target
/// Prints result to stdout
fn unify_cc(focus: &Clause, target: &Clause) {
    let mut focus = focus.clone();
    let mut target = target.clone();

    for l in focus.literals.iter_mut() {
        for other_l in target.literals.iter_mut() {
            if l.neg != other_l.neg {
                let ref mut pred = l.predicate;
                let ref mut other_p = other_l.predicate;

                if pred == other_p {
                    unify_tt(pred, other_p);
                }
            }
        }
    }

    focus.print();
    target.print();
}

/// Performs unification on pred and other
fn unify_tt(pred: Box<RefCell<Predicate>>, other: Box<RefCell<Predicate>>) -> bool {
    let len = pred.terms.len();
    for i in 0..len {
        let t1 = pred.terms.get_mut(i).unwrap();
        let t2 = other.terms.get_mut(i).unwrap();
        unify_terms(pred, other, t1, t2);
    }
    true
}

fn unify_terms(pred: &mut Predicate, other: &mut Predicate,
               t1: &mut Term, t2: &mut Term) -> bool {
    match (t1.get_type(), t2.get_type())  {
        (Constant, Constant) => t1 == t2,
        (Constant, Function) => false,
        (Constant, Variable) => { other.set_var(t1, t2); true },
        (Function, Function) =>
            if t1 != t2 {
                false
            } else {
                let iter = t1.terms.iter_mut().zip(t2.terms.iter_mut());
                for (term1, term2) in iter {
                    if !unify_terms(pred, other, term1, term2) {
                        return false;
                    }
                }
                true
            },
        (Function, Variable) =>
            if t1.contains(t2) {
                false
            } else {
                t2.name = t1.name.clone();
                t2.terms = t1.terms.clone();
                true
            },
        (Function, Constant) => false,
        (Variable, Constant) => { pred.set_var(t2, t1); true },
        (Variable, Function) =>
            if t2.contains(t1) {
                false
            } else {
                t1.name = t2.name.clone();
                t1.terms = t2.terms.clone();
                true
            },
        (Variable, Variable) => { other.set_var(t1, t2); true },
    }
}

fn main() {
    let mut knowledge_base: Vec<Clause> = Vec::new();
    while let Some(input) = read_line() {
        knowledge_base.push(parse_clause(&input));
    }

    unify(knowledge_base);
}
