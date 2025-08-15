use std::collections::HashSet;
use crate::cnf::Lit;

pub trait Assignments {
    /**
     * Determine if there is a satisfying assignment for the given literal.
     */
    fn is_sat(&self, lit: &Lit) -> bool;

    /**
     * Determine if a given literal is unassigned,
     * i.e. it is neither satisfied nor unsatisfied.
     */
    fn is_unassigned(&self, lit: &Lit) -> bool {
        !self.is_sat(lit) && !self.is_sat(&-lit)
    }

    /**
     * Get the set of assigned literals (i.e. literals with satisfying
     * assignments).
     */
    fn get_assignments(&self) -> HashSet<Lit>;
}