#[cfg(test)]
mod tests;

use std::collections::HashSet;
use crate::assignments::Assignments;
use crate::cnf::{Clause, Cnf, Lit, Var};
use crate::trail::{Trail, TrailNode, TrailNodeDecorator};

// Kappa denotes the conflict node
const KAPPA: Lit = 0;

pub fn eval(cnf: &Cnf, assign: &HashSet<Lit>) -> bool {
    cnf.clauses().iter()
        .all(|clause| clause.iter().any(|lit| assign.contains(lit)))
}

pub fn solve(cnf: &mut Cnf) -> Result<HashSet<Lit>, ()> {
    // let mut assign = pre_process(cnf);
    // let mut assign = Assignments::new(cnf.num_vars());
    cdcl(cnf, &mut Trail::new())
}

fn cdcl(cnf: &mut Cnf, trail: &mut Trail)
    -> Result<HashSet<Lit>, ()> {
    loop {
        // Unit prop until we no longer learn a new clause
        loop {
            // Find all unsatisfied clauses
            let unsat_clauses: Vec<&Clause> = cnf.clauses().iter()
                .filter(|clause| is_clause_unsat(clause, trail))
                .collect();

            // Unit prop
            match unit_prop_and_learn(&unsat_clauses, trail) {
                Err(()) => { return Err(()); }, // UNSAT
                Ok(Some(clause)) => { // Learned a new clause
                    cnf.add_clause(clause);
                }
                Ok(None) => { // Propagated without conflicts
                    break;
                }
            }
        }

        // Find a literal to make an arbitrary decision on
        // TODO: Get next unassigned lit from unsat clauses
        let next_choice;
        match get_next_unassigned(cnf, trail) {
            // If there are no unassigned variables, then we're done
            // There will never be an unsat fully-assigned clause at this point
            None => {
                return Ok(trail.get_assignments());
            },
            // Make an arbitrary decision on some undecided variable
            Some(&lit) => {
                next_choice = lit;
            },
        }

        // Assign our guess
        trail.push(next_choice, TrailNodeDecorator::Decision);
    }
}

fn unit_prop_and_learn(unsat_clauses: &Vec<&Clause>,
                       trail: &mut Trail) -> Result<Option<Clause>, ()> {
    match unit_prop(unsat_clauses, trail) {
        // Conflict at level zero is unrecoverable
        Err(()) if trail.dec_level() == 0 => {
            Err(())
        },
        // Learn from a conflict after level zero
        Err(()) => {
            let cut_set = find_uip_cut_set(trail);

            // Construct the reason set, i.e. the set of all nodes in the trail
            // that have an edge connecting them to a member of the B set.
            let mut reason: HashSet<Lit> = HashSet::new();
            cut_set.iter()
                .for_each(|&l| {
                    trail.get_parents(&l).iter()
                        .filter(|p| !cut_set.contains(p))
                        .for_each(|&p| { reason.insert(p); });
                });

            // Make the cut
            cut_set.iter()
                .for_each(|l| trail.remove(l));

            // Learn the reason clause
            let learned = get_inverse_clause(&reason);

            // Backtrack non-chronologically
            backtrack(trail);

            Ok(Some(learned))
        },
        // No conflicts
        Ok(()) => {
            Ok(None)
        },
    }
}

fn backtrack(trail: &mut Trail) {
    // Remove all nodes later than current decision level - 2
    let mut seen_levels = 0;
    loop {
        let top = trail.pop();

        match top {
            None => { break; },
            Some(TrailNode { decorator: TrailNodeDecorator::Decision, ..}) => {
                seen_levels += 1;
            },
            _ => {},
        }

        if seen_levels == 2 {
            break;
        }
    }
}

fn find_uip_cut_set(trail: &Trail) -> HashSet<Lit> {
    let scope = trail.get_latest_decision_children();

    let mut track: HashSet<Lit> = HashSet::new();
    track.insert(KAPPA);

    let mut cut_set: HashSet<Lit> = HashSet::new();

    loop {
        let next = trail.get_latest_in_set(&track);

        cut_set.insert(next);

        track.remove(&next);
        let parents = trail.get_parents(&next);
        parents.iter()
            .filter(|p| scope.contains(p))
            .for_each(|&p| {
                track.insert(p);
            });

        if track.len() == 1 {
            break;
        }
    }

    cut_set
}

fn unit_prop<'a>(unsat_clauses: &Vec<&Clause>,
                 trail: &mut Trail) -> Result<(), ()> {
    // Perform unit propagation until there are no unit clauses
    loop {
        // Refine our search to only the unit clauses
        let unit_clauses: Vec<&&Clause> = unsat_clauses.iter()
            // Re-filter unsat clauses since this will change between iterations
            .filter(|clause| is_clause_unsat(clause, trail))
            .filter(|clause| is_clause_unit(clause, trail))
            .collect();

        match unit_clauses.get(0) {
            Some(&&clause) => {
                let unit = clause.iter()
                    .find(|&lit|
                        !trail.is_sat(lit) && !trail.is_sat(&-lit))
                    .unwrap();

                // Check if there is a conflicting unit literal
                let conflicting_clause = unit_clauses.iter()
                    .find(|clause|
                        clause.iter()
                            .any(|lit|
                                !trail.is_sat(lit) &&
                                    !trail.is_sat(&-lit) &&
                                    unit.eq(&-lit)));

                match conflicting_clause {
                    Some(&&conflicting) => {
                        trail.push(*unit, TrailNodeDecorator::Clause(clause.clone()));
                        trail.push(KAPPA, TrailNodeDecorator::Clause(conflicting.clone()));
                        break Err(());
                    }
                    None => {}, // Empty
                }

                // There is no conflict, so add the satisfying assignment
                trail.push(*unit, TrailNodeDecorator::Clause(clause.clone()));
            },
            None => break Ok(()),
        }
    }
}

// fn pre_process(cnf: &Cnf) -> Assignments {
//     let mut assign = Assignments::new(cnf.num_vars());
//     pure_lit_assign(cnf, &mut assign);
//     assign
// }

// fn pure_lit_assign(cnf: &Cnf, assign: &mut Assignments) {
//     let mut seen_lits: Vec<bool> = vec![false; cnf.num_vars() * 2];
//     cnf.clauses().iter()
//         .filter(|clause| is_clause_unsat(clause, assign))
//         .flatten()
//         .for_each(|lit| {
//             let ind =
//                 (lit.abs() - 1) as usize * 2 + if lit < &0 { 0 } else { 1 };
//             seen_lits[ind] = true;
//         });
//
//     let mut i = 0;
//     while i < seen_lits.len() {
//         let var = i as i32 / 2 + 1;
//         // Pure, negative
//         if seen_lits[i] && !seen_lits[i + 1] {
//             assign.put(&-var);
//         }
//         // Pure, positive
//         else if !seen_lits[i] && seen_lits[i + 1] {
//             assign.put(&var);
//         }
//         // Unused
//         else if !seen_lits[i] && !seen_lits[i + 1] {
//             assign.put(&-var);
//         }
//
//         i += 2;
//     }
// }

fn get_inverse_clause(set: &HashSet<Lit>) -> Clause {
    set.iter()
        .map(|l| -l)
        .collect()
}

fn get_next_unassigned<'a>(cnf: &'a Cnf,
                           assign: &dyn Assignments) -> Option<&'a Lit> {
    cnf.clauses().iter()
        .filter(|clause| is_clause_unsat(clause, assign))
        .flatten()
        .find(|&lit| !assign.is_sat(lit) && !assign.is_sat(&-lit))
}

fn is_clause_unsat(clause: &Clause, assign: &dyn Assignments) -> bool {
    !clause.iter().any(|lit| assign.is_sat(lit))
}

fn is_clause_unit(clause: &Clause, assign: &dyn Assignments) -> bool {
    clause.iter()
        .filter(|&lit| !assign.is_sat(lit) && !assign.is_sat(&-lit))
        .count() == 1
}

/**
 * Given a literal, get the variable.
 */
fn var_of_lit(lit: &Lit) -> Var {
    lit.abs()
}