#[cfg(test)]
mod tests;

use std::fmt::{Display, Formatter};
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
    /**
     * Create a new set of assignments with capacity for the given number of
     * variables.
     */
    pub fn new(num_vars: usize) -> Self {
        Assignments {
            num_vars,
            num_assigned: 0,
            var_assign: vec![UNASSIGNED; num_vars],
        }
    }

    /**
     * Create a new set of assignments that satisfies the given set of literals.
     */
    pub fn from(lits: Vec<Lit>, num_vars: usize) -> Self {
        let mut assign_vec = vec![UNASSIGNED; num_vars];
        lits.iter()
            .for_each(|lit| {
                assign_vec[Self::get_ind(lit.abs())] =
                    if lit > &0 { TRUE } else { FALSE }
            });

        Assignments {
            num_vars,
            num_assigned: lits.len(),
            var_assign: assign_vec,
        }
    }

    /**
     * Determine if a given literal is satisfied by the assignments.
     */
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

    /**
     * Determine if the given variable is assigned.
     */
    pub fn is_assigned(&self, var: &Var) -> bool {
        self.var_assign[Self::get_ind(*var)] != UNASSIGNED
    }

    /**
     * Write the satisfying assignment of the given literal to the assignments.
     */
    pub fn put(&mut self, lit: Lit) {
        let ind = Self::get_ind(lit);

        if self.var_assign[ind] != UNASSIGNED {
            panic!()
        }

        let value = if lit > 0 { TRUE } else { FALSE };
        self.num_assigned += 1;
        self.var_assign[ind] = value;
    }

    fn get_ind(lit: i32) -> usize {
        lit.abs() as usize - 1
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

impl Display for Assignments {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.var_assign.len() {
            f.write_fmt(format_args!("{}{}{}",
                if self.var_assign[i] == FALSE { "-" } else { "" },
                i + 1,
                if i < self.var_assign.len() - 1 { " " } else { "" })
            ).expect("An error occurred while formatting");
        }

        Ok(())
    }
}