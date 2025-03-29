#[cfg(test)]
mod tests;

use crate::cnf::{Clause, Cnf, Lit, Var, Assignments};

pub fn eval(cnf: &Cnf, assign: &Assignments) -> bool {
    cnf.iter()
        .all(|clause| clause.iter().any(|lit| assign.is_sat(lit)))
}

pub fn dpll(cnf: &Cnf, num_vars: usize) -> Result<Assignments, ()> {
    fn aux(cnf: &Cnf, assign: &mut Assignments, num_vars: usize) -> Result<Assignments, ()> {
        match unit_prop(cnf, assign) {
            Err(_) => Err(()),
            Ok(()) => {
                if !assign.has_unassigned() {
                    return Ok(assign.clone())
                }

                let next_var = find_unassigned(&assign, num_vars).unwrap();
                let mut assign_a = assign.clone();
                assign_a.put(next_var);
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

fn find_unassigned(assign: &Assignments, num_vars: usize) -> Option<Var> {
    for v in 1..(num_vars + 1) {
        if !assign.is_assigned(&(v as Var)) {
            return Some(v as Var)
        }
    }

    None
}

fn unit_prop(cnf: &Cnf, assign: &mut Assignments) -> Result<(), Var> {
    loop {
        let units: Vec<Lit> = cnf.iter()
            // Ignore clauses that are already satisfied by another assignment
            .filter(|clause| !clause.iter().any(|lit| assign.is_sat(lit)))
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

fn get_unit_unassigned(clause: &Clause, assign: &Assignments) -> Option<Lit> {
    let unassigned: Vec<&Lit> = clause.iter()
        .filter(|lit| !assign.is_assigned(lit))
        .collect();

    if unassigned.len() != 1 {
        None
    }
    else {
        Some(**unassigned.get(0).unwrap())
    }
}

fn var_of_lit(lit: &Lit) -> Var {
    lit.abs()
}