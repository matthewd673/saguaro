use super::*;
use rstest::*;

#[rstest]
#[case(0, vec![], vec![], true)]
#[case(1, vec![vec![1], vec![]], vec![1], false)]
#[case(1, vec![vec![1]], vec![1], true)]
#[case(1, vec![vec![1]], vec![], false)]
#[case(1, vec![vec![-1]], vec![-1], true)]
#[case(1, vec![vec![-1]], vec![1], false)]
#[case(1, vec![vec![1, -1]], vec![1], true)]
#[case(1, vec![vec![1], vec![-1]], vec![1], false)]
#[case(3, vec![vec![1, 2], vec![-2], vec![3]], vec![1, -2, 3], true)]
fn eval_test(#[case] num_vars: usize,
             #[case] clauses: Vec<Clause>,
             #[case] assign_lits: Vec<Lit>,
             #[case] expected: bool) {
    let cnf = Cnf::new(clauses, num_vars);
    let assign = Assignments::from(assign_lits, num_vars);

    assert_eq!(expected, eval(&cnf, &assign));
}

#[rstest]
#[case(0, vec![], vec![], vec![])]
#[case(1, vec![vec![1]], vec![], vec![1])]
#[case(1, vec![vec![1]], vec![1], vec![1])]
#[case(1, vec![vec![-1]], vec![], vec![-1])]
#[case(3, vec![vec![1, 2, 3], vec![1]], vec![], vec![1])]
#[case(3, vec![vec![1, 2, 3], vec![1]], vec![], vec![1])]
#[case(3, vec![vec![1, 2, 3], vec![1]], vec![3], vec![1, 3])] // NOTE: 2 can have any value
#[case(3, vec![vec![1, 2, 3], vec![-1], vec![-2]], vec![], vec![-1, -2, 3])]
#[case(3, vec![vec![1, 2], vec![1, 2, 3]], vec![], vec![])]
#[case(4, vec![vec![1], vec![-2], vec![3, -4]], vec![], vec![1, -2])]
fn unit_prop_test_success(#[case] num_vars: usize,
                          #[case] clauses: Vec<Clause>,
                          #[case] assign_lits: Vec<Lit>,
                          #[case] exp_assign_lits: Vec<Lit>) {
    let cnf = Cnf::new(clauses, num_vars);
    let assign = Assignments::from(assign_lits, num_vars);
    let exp_assign = Assignments::from(exp_assign_lits, num_vars);

    let mut m_assign = assign.clone();
    assert_eq!(Ok(()), unit_prop(&cnf, &mut m_assign));
    assert_eq!(exp_assign, m_assign);
}

#[rstest]
#[case(1, vec![vec![1], vec![-1]], vec![])]
#[case(2, vec![vec![1, 2], vec![-1], vec![-2]], vec![])]
#[case(2, vec![vec![1], vec![-1, 2], vec![-2]], vec![])]
fn unit_prop_test_conflict(#[case] num_vars: usize,
                           #[case] clauses: Vec<Clause>,
                           #[case] assign_lits: Vec<Lit>) {
    let cnf = Cnf::new(clauses, num_vars);
    let assign = Assignments::from(assign_lits, num_vars);

    let mut m_assign = assign.clone();
    assert_eq!(true, matches!(unit_prop(&cnf, &mut m_assign), Err(_)));
}

#[rstest]
#[case(0, vec![], vec![], true)]
#[case(1, vec![1], vec![], true)]
#[case(1, vec![1], vec![1], false)]
#[case(1, vec![1], vec![-1], true)]
#[case(3, vec![1, 2, 3], vec![2], false)]
#[case(3, vec![1, 2, 3], vec![-2], true)]
fn is_clause_unsat_test(#[case] num_vars: usize,
                        #[case] clause: Clause,
                        #[case] assign_lits: Vec<Lit>,
                        #[case] expected: bool) {
    let assign = Assignments::from(assign_lits, num_vars);

    assert_eq!(expected, is_clause_unsat(&clause, &assign));
}

#[rstest]
#[case(0, vec![], vec![], None)]
#[case(1, vec![1], vec![], Some(1))]
#[case(1, vec![-1], vec![], Some(-1))]
#[case(2, vec![1, 2], vec![], None)]
#[case(2, vec![1, 2], vec![1], Some(2))]
#[case(2, vec![1, 2], vec![2], Some(1))]
#[case(2, vec![1, 2], vec![-1], Some(2))]
#[case(4, vec![1, 2, 3, 4], vec![1, 3, 4], Some(2))]
fn get_unit_unassigned_test(#[case] num_vars: usize,
                            #[case] clause: Clause,
                            #[case] assign_lits: Vec<Lit>,
                            #[case] expected: Option<Lit>) {
    let assign = Assignments::from(assign_lits, num_vars);

    assert_eq!(expected, get_unit_unassigned(&clause, &assign));
}

#[rstest]
#[case(1, 1)]
#[case(-1, 1)]
fn var_of_lit_test(#[case] lit: Lit, #[case] expected: Var) {
    assert_eq!(expected, var_of_lit(&lit));
}
