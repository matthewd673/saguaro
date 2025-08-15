use rstest::*;
use saguaro::cnf::{Clause, Cnf};

#[rstest]
#[case(Cnf::new(vec![vec![1, 2]], 2), vec![])]
#[case(Cnf::new(vec![], 0), vec![1, 2])]
fn add_clause_test(#[case] mut cnf: Cnf, #[case] clause: Clause) {
    let clauses = cnf.clauses().clone();
    cnf.add_clause(clause.clone());

    assert_eq!(clauses.len() + 1, cnf.clauses().len());
    for i in 0..clauses.len() {
        assert_eq!(clauses.get(i), cnf.clauses().get(i));
    }
    assert!(clause.eq(cnf.clauses().last().unwrap()));
}