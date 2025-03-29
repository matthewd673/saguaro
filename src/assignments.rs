#[cfg(test)]
mod tests;

use crate::cnf::{Lit, Var};

const UNASSIGNED: u8 = 0;
const TRUE: u8 = 1;
const FALSE: u8 = 2;

#[derive(Debug)]
pub struct Assignments {
    num_vars: usize,
    num_assigned: usize,
    var_assign: Vec<u8>,
}

impl Assignments {
    pub fn new(num_vars: usize) -> Self {
        Assignments {
            num_vars,
            num_assigned: 0,
            var_assign: vec![UNASSIGNED; num_vars],
        }
    }

    pub fn from<const N: usize>(num_vars: usize, lits: [Lit; N]) -> Self {
        let mut assign_vec = vec![UNASSIGNED; num_vars];
        lits.iter()
            .for_each(|lit| {
                let var = lit.abs();
                assign_vec[Self::get_ind(var)] = if lit > &0 { TRUE } else { FALSE }
            });

        Assignments {
            num_vars: assign_vec.len(),
            num_assigned: lits.len(),
            var_assign: assign_vec,
        }
    }

    pub fn is_sat(&self, lit: &Lit) -> bool {
        let var = lit.abs();
        let ind = Self::get_ind(var);

        if lit < &0 {
            self.var_assign[ind] == FALSE
        }
        else {
            self.var_assign[ind] == TRUE
        }
    }

    pub fn is_assigned(&self, var: &Var) -> bool {
        self.var_assign[Self::get_ind(*var)] != UNASSIGNED
    }

    pub fn put(&mut self, lit: Lit) {
        let ind = Self::get_ind(lit.abs());

        if self.var_assign[ind] != UNASSIGNED {
            panic!()
        }

        let value = if lit > 0 { TRUE } else { FALSE };
        self.num_assigned += 1;
        self.var_assign[ind] = value;
    }

    pub fn has_unassigned(&self) -> bool {
        self.num_assigned != self.num_vars
    }

    fn get_ind(var: Var) -> usize {
        var as usize - 1
    }
}

impl Clone for Assignments {
    fn clone(&self) -> Self {
        Assignments {
            num_vars: self.num_vars,
            num_assigned: self.num_assigned,
            var_assign: self.var_assign.clone(),
        }
    }
}

impl PartialEq for Assignments {
    fn eq(&self, other: &Self) -> bool {
        self.var_assign.eq(&other.var_assign)
    }
}
