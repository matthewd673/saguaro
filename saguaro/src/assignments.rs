use std::collections::HashSet;
use crate::cnf::Lit;

pub trait Assignments {
    /**
     * Determine if there is a satisfying assignment for the given literal.
     */
    fn is_sat(&self, lit: &Lit) -> bool;

    /**
     * Get the set of assigned literals (i.e. literals with satisfying
     * assignments).
     */
    fn get_assignments(&self) -> HashSet<Lit>;
}