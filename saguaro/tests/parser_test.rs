use rstest::*;
use saguaro::cnf::Cnf;
use saguaro::parser::parse;

#[rstest]
#[case("", None)]
#[case("bad input", None)]
#[case("p cnf 12", None)]
#[case("p cnf 0 0", None)]
#[case("p cnf 0 0\naaa", None)]
#[case("p cnf 2 1\n1 2 0", Some(Cnf::new(vec![vec![1, 2]], 2)))]
#[case("c with comment p cnf 5 1\np cnf 2 1\nc another comment\n1 2 0",
       Some(Cnf::new(vec![vec![1, 2]], 2)))]
#[case("p cnf 3 3 1 2 3 0 -2 1 0 -3 -1 0",
       Some(Cnf::new(vec![vec![1, 2, 3], vec![-2, 1], vec![-3, -1]], 3)))]
fn parse_test(#[case] input: &str, #[case] expected: Option<Cnf>) {
    let actual = parse(String::from(input));

    match (expected, actual) {
        (None, None) => assert!(true),
        (Some(a), Some(b)) => assert_cnf_eq(&a, &b),
        _ => assert!(false),
    }
}

fn assert_cnf_eq(a: &Cnf, b: &Cnf) {
    assert_eq!(a.num_vars(), b.num_vars());
    assert_eq!(a.clauses(), b.clauses());
}
