use super::*;
use rstest::*;

#[rstest]
#[case(Assignments::from(vec![], 0), "")]
#[case(Assignments::from(vec![1], 1), "1")]
#[case(Assignments::from(vec![-1], 1), "-1")]
#[case(Assignments::from(vec![1, -2, 3], 3), "1 -2 3")]
fn to_string_test(#[case] assign: Assignments, #[case] expected: &str) {
    assert_eq!(expected, assign.to_string());
}