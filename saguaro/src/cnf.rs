pub type Var = i32;
pub type Lit = i32;
pub type Clause = Vec<Lit>;

pub struct Cnf {
    clauses: Vec<Clause>,
    num_vars: usize,
}

impl Cnf {
    pub fn new(clauses: Vec<Clause>, num_vars: usize) -> Self {
        Cnf {
            clauses,
            num_vars,
        }
    }

    pub fn clauses(&self) -> &Vec<Clause> {
        &self.clauses
    }

    pub fn add_clause(&mut self, clause: Clause) {
        self.clauses.push(clause);
    }

    pub fn num_vars(&self) -> usize {
        self.num_vars
    }
}