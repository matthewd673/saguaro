use std::ptr::null_mut;
use super::*;
use rstest::*;

#[rstest]
#[case(vec![], Assignments::new(0), true)]
#[case(vec![vec![1], vec![]], Assignments::from(1, [1]), false)]
#[case(vec![vec![1]], Assignments::from(1, [1]), true)]
#[case(vec![vec![1]], Assignments::new(0), false)]
#[case(vec![vec![-1]], Assignments::from(1, [-1]), true)]
#[case(vec![vec![-1]], Assignments::from(1, [1]), false)]
#[case(vec![vec![1, -1]], Assignments::from(1, [1]), true)]
#[case(vec![vec![1], vec![-1]], Assignments::from(1, [1]), false)]
#[case(vec![vec![1, 2], vec![-2], vec![3]], Assignments::from(3, [1, -2, 3]), true)]
fn eval_test(#[case] cnf: Cnf, #[case] assign: Assignments, #[case] expected: bool) {
    assert_eq!(expected, eval(&cnf, &assign));
}

#[rstest]
#[case(Assignments::new(0), 0, None)]
#[case(Assignments::new(0), 1, Some(1))]
#[case(Assignments::from(3, [1, 2, 3]), 3, None)]
#[case(Assignments::from(3, [1, 2, 3]), 4, Some(4))]
#[case(Assignments::from(3, [2, 3, 4]), 4, Some(1))]
fn find_unassigned_test(#[case] assign: Assignments,
                        #[case] num_vars: usize,
                        #[case] expected: Option<Var>) {
    assert_eq!(expected, find_unassigned(&assign, num_vars));
}

#[rstest]
#[case(vec![], Assignments::new(0), Assignments::new(0))]
#[case(vec![vec![1]], Assignments::new(0), Assignments::from(1, [1]))]
#[case(vec![vec![1]], Assignments::from(1, [1]), Assignments::from(1, [1]))]
#[case(vec![vec![-1]], Assignments::new(0), Assignments::from(1, [-1]))]
#[case(vec![vec![1, 2, 3], vec![1]], Assignments::new(0), Assignments::from(1, [1]))]
#[case(vec![vec![1, 2, 3], vec![1]], Assignments::from(3, []), Assignments::from(1, [1]))]
#[case(vec![vec![1, 2, 3], vec![1]], Assignments::from(3, [3]), Assignments::from(3, [1, 3]))] // NOTE: 2 can have any value
#[case(vec![vec![1, 2, 3], vec![-1], vec![-2]], Assignments::new(0), Assignments::from(3, [-1, -2, 3]))]
#[case(vec![vec![1, 2], vec![1, 2, 3]], Assignments::new(0), Assignments::from(3, []))]
#[case(vec![vec![1], vec![-2], vec![3, -4]], Assignments::new(0), Assignments::from(4, [1, -2]))]
fn unit_prop_test_success(#[case] cnf: Cnf,
                          #[case] assign: Assignments,
                          #[case] exp_assign: Assignments) {
    let mut m_assign = assign.clone();
    assert_eq!(Ok(()), unit_prop(&cnf, &mut m_assign));
    assert_eq!(exp_assign, m_assign);
}

#[rstest]
#[case(vec![vec![1], vec![-1]], Assignments::new(0))]
#[case(vec![vec![1, 2], vec![-1], vec![-2]], Assignments::new(0))]
#[case(vec![vec![1], vec![-1, 2], vec![-2]], Assignments::new(0))]
fn unit_prop_test_conflict(#[case] cnf: Cnf, #[case] assign: Assignments) {
    let mut m_assign = assign.clone();
    assert_eq!(true, matches!(unit_prop(&cnf, &mut m_assign), Err(_)));
}

#[rstest]
#[case(vec![], Assignments::new(0), None)]
#[case(vec![1], Assignments::new(0), Some(1))]
#[case(vec![-1], Assignments::new(0), Some(-1))]
#[case(vec![1, 2], Assignments::new(0), None)]
#[case(vec![1, 2], Assignments::from(1, [1]), Some(2))]
#[case(vec![1, 2], Assignments::from(1, [2]), Some(1))]
#[case(vec![1, 2], Assignments::from(1, [-1]), Some(2))]
#[case(vec![1, 2, 3, 4], Assignments::from(4, [1, 3, 4]), Some(2))]
fn get_unit_unassigned_test(#[case] clause: Clause,
                            #[case] assign: Assignments,
                            #[case] expected: Option<Lit>) {
    assert_eq!(expected, get_unit_unassigned(&clause, &assign));
}

#[rstest]
#[case(1, 1)]
#[case(-1, 1)]
fn var_of_lit_test(#[case] lit: Lit, #[case] expected: Var) {
    assert_eq!(expected, var_of_lit(&lit));
}
