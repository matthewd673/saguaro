#[cfg(test)]
mod tests;

use crate::cnf::{Clause, Cnf, Lit, Var};
use crate::assignments::Assignments;

pub fn eval(cnf: &Cnf, assign: &Assignments) -> bool {
    cnf.iter()
        .all(|clause| clause.iter().any(|lit| assign.is_sat(lit)))
}

pub fn dpll(cnf: &Cnf, num_vars: usize) -> Result<Assignments, ()> {
    fn aux(cnf: &Cnf, assign: &mut Assignments, num_vars: usize) -> Result<Assignments, ()> {
        match unit_prop(cnf, assign) {
            Err(_) => Err(()),
            Ok(()) => {
                let next_unassigned = cnf.iter()
                    .filter(|clause| is_clause_unsat(clause, assign))
                    .flatten()
                    .find(|lit| !assign.is_assigned(&var_of_lit(lit)));

                if matches!(next_unassigned, None) {
                    return Ok(assign.clone())
                }

                let next_var = next_unassigned.unwrap();
                let mut assign_a = assign.clone();
                assign_a.put(*next_var);
                let mut assign_b = assign.clone();
                assign_b.put(-next_var);

                let branch_a = aux(cnf, &mut assign_a, num_vars);
                let branch_b = aux(cnf, &mut assign_b, num_vars);

                match (branch_a, branch_b) {
                    (Ok(a), Ok(_)) => Ok(a),
                    (Ok(a), Err(())) => Ok(a),
                    (Err(()), Ok(b)) => Ok(b),
                    (Err(()), Err(())) => Err(()),
                }
            }
        }
    }

    let mut assign = Assignments::new(num_vars);
    aux(cnf, &mut assign, num_vars)
}

fn unit_prop(cnf: &Cnf, assign: &mut Assignments) -> Result<(), Var> {
    let unsat_clauses: Vec<&Clause> = cnf.iter()
        // Ignore clauses that are already satisfied by another assignment
        .filter(|clause| is_clause_unsat(clause, assign))
        .collect();

    loop {
        let units: Vec<Lit> = unsat_clauses.iter()
            // Re-compute new subset of unsat clauses since this can cascade
            .filter(|clause| is_clause_unsat(clause, assign))
            // If the clause has a single unassigned literal, return it
            .map(|clause| get_unit_unassigned(clause, &assign))
            // Ignore clauses that don't have exactly one unassigned literal
            .filter(|u| matches!(u, Some(_)))
            .map(|u| u.unwrap())
            .collect();

        match units.get(0) {
            Some(u) => {
                // Check for a conflict before propagating
                if units.contains(&-u) {
                    break Err(var_of_lit(&u))
                }

                assign.put(*u);
            },
            None => break Ok(()),
        }
    }
}

fn is_clause_unsat(clause: &Clause, assign: &Assignments) -> bool {
    !clause.iter().any(|lit| assign.is_sat(lit))
}

fn get_unit_unassigned(clause: &Clause, assign: &Assignments) -> Option<Lit> {
    let mut all_unassigned = clause.iter()
        .filter(|lit| !assign.is_assigned(&var_of_lit(lit)));

    // If this is a unit clause, then the first unassigned literal is the one we care about.
    // If there are more items in the iterator after the first, this isn't a unit clause.
    let first_unassigned = all_unassigned.next();
    if !matches!(first_unassigned, None) && matches!(all_unassigned.next(), None) {
        Some(*first_unassigned.unwrap())
    }
    else {
        None
    }
}

fn var_of_lit(lit: &Lit) -> Var {
    lit.abs()
}