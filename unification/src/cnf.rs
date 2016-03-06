use ::regex::Regex;

#[derive(Clone, Debug)]
pub struct Clause {
    pub literals: Vec<Literal>,
}

impl Clause {
    pub fn new() -> Clause {
        Clause {
            literals: Vec::new(),
        }
    }
    pub fn print(&self) {
        let mut iter = self.literals.iter().peekable();
        loop {
            match iter.next() {
                Some(lit) => {
                    lit.print();
                    if iter.peek().is_some() {
                        print!(" | ");
                    }
                },
                None => break,
            }
        }
        println!("");
    }
}

#[derive(Clone, Debug)]
pub struct Literal {
    pub predicate: Predicate,
    pub neg: bool,
}

impl Literal {
    pub fn new(p: Predicate, neg: bool) -> Literal {
        Literal {
            predicate: p,
            neg: neg
        }
    }
    pub fn print(&self) {
        if self.neg {
            print!("-")
        }
        self.predicate.print();
    }
}

#[derive(Clone, Debug)]
pub struct Predicate {
    pub name: String,
    pub terms: Vec<Term>,
}

impl Predicate {
    pub fn new(n: String) -> Predicate {
        Predicate {
            name: n,
            terms: Vec::new(),
        }
    }
    pub fn set_var(&mut self, constant: &Term, var: &Term) {
        for term in self.terms.iter_mut() {
            match term.get_type() {
                TermType::Function => term.set_var(constant, var),
                TermType::Variable => if term == var {
                    term.name = constant.name.clone();
                },
                TermType::Constant => println!("Something broke."),
            }
        }
    }
    pub fn print(&self) {
        print!("{}(", self.name);
        let mut iter = self.terms.iter().peekable();
        loop {
            match iter.next() {
                Some(term) => {
                    term.print();
                    if iter.peek().is_some() {
                        print!(", ");
                    }
                },
                None => break,
            }
        }
        print!(")");
    }
}

impl PartialEq for Predicate {
    fn eq(&self, other: &Predicate) -> bool {
        self.name == *other.name &&
            self.terms.len() == other.terms.len()
    }
}

pub enum TermType {
    Constant,
    Variable,
    Function
}

#[derive(Clone, Debug)]
pub struct Term {
    pub name: String,
    pub terms: Vec<Term>
}

impl Term {
    pub fn new(n: String) -> Term {
        Term {
            name: n,
            terms: Vec::new()
        }
    }
    pub fn get_type(&self) -> TermType {
        if self.terms.len() > 0 {
            return TermType::Function
        }
        match Regex::new(r"^[:upper:].*$").unwrap().is_match(&self.name) {
            true => TermType::Constant,
            false => TermType::Variable,
        }
    }
    pub fn contains(&self, t: &Term) -> bool {
        for term in self.terms.iter() {
            match term.get_type() {
                TermType::Function => if term.contains(t) {
                    return true;
                },
                _ => if term == t {
                    return true;
                }
            }
        }
        false
    }
    pub fn set_var(&mut self, constant: &Term, var: &Term) {
        match self.get_type() {
            TermType::Function => {
                for term in self.terms.iter_mut() {
                    term.set_var(constant, var);
                }
            },
            TermType::Variable => if self == var {
                self.name = constant.name.clone()
            },
            TermType::Constant => println!("Something died."),
        }
    }

    pub fn print(&self) {
        print!("{}", self.name);
        if self.terms.len() > 0 {
            print!("(");
            let mut iter = self.terms.iter().peekable();
            loop {
                match iter.next() {
                    Some(term) => {
                        term.print();
                        if iter.peek().is_some() {
                            print!(", ");
                        }
                    },
                    None => break,
                }
            }
            print!(")");
        }
    }
}

impl PartialEq for Term {
    fn eq(&self, other: &Term) -> bool {
        self.name == *other.name &&
            self.terms.len() == other.terms.len()
    }
}
