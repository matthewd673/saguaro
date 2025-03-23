use std::fmt::{Display, Formatter};

pub type Var = i32;
pub type Lit = i32;
pub type Clause = Vec<Lit>;
pub type Cnf = Vec<Clause>;