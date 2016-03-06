use std::io::BufRead;

static mut BRANCHES: usize = 0;

fn read_preamble() -> Option<(usize, usize)> {
    let stdin = std::io::stdin();
    let mut line = String::new();

    while let Ok(size) = stdin.lock().read_line(&mut line) {
        let buff = line.clone();
        line.clear();
        if size == 0 {
            // Got EOF without falling into p directive
            break;
        }

        let words: Vec<&str> = buff.split_whitespace().collect();
        match words.get(0) {
            Some(&"c") => continue,
            Some(&"p") => {
                match words.get(1) {
                    Some(&"cnf") => {},
                    _ => {
                        println!("error with p directive");
                        continue
                    },
                };
                let (vars, clauses) = match (words.get(2), words.get(3)) {
                    (Some(vars), Some(clauses)) => (vars, clauses),
                    _ => {
                        println!("error with p directive");
                        continue
                    },
                };

                let vars_num = vars.parse::<usize>().unwrap();
                let clauses_num = clauses.parse::<usize>().unwrap();
                return Some((vars_num, clauses_num));
            },
            Some(x) => println!("Unknown {}", x),
            None => println!("Input error"),
        }
    }
    None
}

/// Takes utf8 from stdin until a 0.
fn read_clause() -> Option<String> {
    let stdin = std::io::stdin();
    let mut buff = Vec::new();
    match stdin.lock().read_until(b'0', &mut buff) {
        Ok(0) => return None,
        Ok(_) => {},
        Err(_) => return None,
    };
    buff.pop(); // Drop ending 0

    // Ensure consuming beyond numbers with 0, i.e. 10, 20
    while !buff.ends_with(&[b' ']) {
        buff.push(b'0');
        let new_buff = read_clause().unwrap();
        let other = new_buff.as_bytes();
        for i in 0..other.len() {
            buff.push(other[i]);
        }
    }

    match std::str::from_utf8(&buff) {
        Ok(s) => Some(String::from(s)),
        _ => None
    }
}

/// Converts a string into Boolean logic.
fn gen_clause(line: String) -> Vec<isize> {
    let mut clause = Vec::new();
    for number in line.split_whitespace() {
        let num = number.parse::<isize>().unwrap();
        clause.push(num);
    }
    clause
}


fn set_variable(clauses: &mut Vec<Vec<isize>>, val: isize) {
    clauses.retain(|x| !x.contains(&val));
    for clause in clauses.iter_mut() {
        clause.retain(|&x| x != (val * -1));
    }
}

fn unit_val(clauses: &Vec<Vec<isize>>) -> isize {
    for clause in clauses.iter() {
        if clause.len() == 1 {
            return *(clause.get(0).unwrap());
        }
    }
    0
}

fn unit_propagate(clauses: &mut Vec<Vec<isize>>, solution: &mut Vec<isize>) {
    //println!("propagateing...");
    let item = unit_val(clauses);
    if item == 0 {
        //println!("No units");
        return;
    } else {
        unsafe { BRANCHES += 1; }
        //println!("pushing {}", item);
        solution.push(item);
        set_variable(clauses, item);
    }
    unit_propagate(clauses, solution);
}

fn dll(vars_num: usize, index: isize, clauses: &mut Vec<Vec<isize>>, solution: &mut Vec<isize>) -> bool {
    //println!("Iter {:?}", clauses);
    unsafe { BRANCHES += 1; }
    unit_propagate(clauses, solution);
    //println!("{:?}", clauses);
    if clauses.len() == 0 {
        return true;
    } else {
        for clause in clauses.iter() {
            if clause.len() == 0 {
                solution.pop();
                //println!("Empty clause");
                return false;
            }
        }
    }

    let mut old_clauses = clauses.clone();
    set_variable(clauses, index);
    solution.push(index);
    //println!("set true{:?}", solution);
    if dll(vars_num, index + 1, clauses, solution) {
        return true;
    }


    solution.pop();
    //println!("true failed{:?}", solution);
    set_variable(&mut old_clauses, -index);
    solution.push(-index);
    //println!("set false{:?}", solution);
    if dll(vars_num, index + 1, &mut old_clauses, solution) {
        return true;
    }

    //println!("false failed.");

    solution.pop();
    //println!("unset false{:?}", solution);
    return false;
}

fn main() {
    let (vars_num, clauses_num) = match read_preamble() {
        Some((v, c)) => (v, c),
        None => { println!("Input error."); return },
    };

    let mut clauses: Vec<Vec<isize>> = Vec::new();
    for _ in 0..clauses_num {
        clauses.push(gen_clause(read_clause().unwrap()));
    }

    let mut solution = Vec::new();
    let solved = dll(vars_num, 1, &mut clauses, &mut solution);
    println!("s cnf {} {} {}", if solved { 1 } else { 0 }, vars_num, clauses_num);
    if solved {
        for sol in solution {
            println!("v {}", sol);
        }
    }
    unsafe { println!("{} branching nodes explored", BRANCHES); }
}
