use std::collections::HashSet;

pub type Var = i32;
pub type Lit = i32;
pub type Clause = Vec<Lit>;
pub type Cnf = Vec<Clause>;

#[derive(Debug)]
pub struct Assignments {
    num_vars: usize,
    assigned_lits: HashSet<Lit>,
}

impl Assignments {
    pub fn new(num_vars: usize) -> Self {
        Assignments {
            num_vars,
            assigned_lits: HashSet::with_capacity(num_vars),
        }
    }

    pub fn from<const N: usize>(num_vars: usize, assigned_lits: [Lit; N]) -> Self {
        Assignments {
            num_vars,
            assigned_lits: HashSet::from(assigned_lits),
        }
    }

    pub fn is_sat(&self, lit: &Lit) -> bool {
        self.assigned_lits.contains(lit)
    }

    pub fn is_assigned(&self, var: &Var) -> bool {
        self.assigned_lits.contains(var) || self.assigned_lits.contains(&-var)
    }

    pub fn put(&mut self, lit: Lit) -> bool {
        if self.assigned_lits.contains(&-lit) {
            panic!()
        }

        self.assigned_lits.insert(lit)
    }

    pub fn has_unassigned(&self) -> bool {
        self.assigned_lits.len() != self.num_vars
    }
}

impl Clone for Assignments {
    fn clone(&self) -> Self {
        Assignments {
            num_vars: self.num_vars,
            assigned_lits: self.assigned_lits.clone(),
        }
    }
}

impl PartialEq for Assignments {
    fn eq(&self, other: &Self) -> bool {
        self.assigned_lits.eq(&other.assigned_lits)
    }
}