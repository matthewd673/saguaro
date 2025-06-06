use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::*;
use saguaro::{solver};
use saguaro::cnf::{Cnf, Clause, Lit};

#[wasm_bindgen]
pub struct Formula {
    expr: And,
}

#[wasm_bindgen]
pub struct And {
    clauses: Vec<Or>,
}

#[wasm_bindgen]
pub struct Or {
    lits: Vec<Literal>,
}

#[wasm_bindgen]
pub struct Literal {
    var: String,
    negated: bool,
}

#[wasm_bindgen]
pub fn formula(expr: And) -> Formula {
    Formula { expr }
}

#[wasm_bindgen]
pub fn and(clauses: Vec<Or>) -> And {
    And { clauses }
}

#[wasm_bindgen]
pub fn or(lits: Vec<Literal>) -> Or {
    Or { lits }
}

#[wasm_bindgen]
pub fn bool(name: String) -> Literal {
    Literal { var: name, negated: false }
}

#[wasm_bindgen]
pub fn not(lit: Literal) -> Literal {
    Literal { var: lit.var, negated: !lit.negated }
}

#[wasm_bindgen]
pub fn solve(formula: Formula) -> js_sys::Object {
    let (mut cnf, var_map) = formula_to_cnf(&formula);
    let solution = solver::solve(&mut cnf);
    solution_to_js_obj(&solution, &var_map)
}

fn formula_to_cnf(formula: &Formula) -> (Cnf, HashMap<i32, String>) {
    let mut var_to_ind_map = HashMap::new();
    let mut ind_to_var_map = HashMap::new();
    let clauses: Vec<Clause> = formula.expr.clauses.iter()
        .map(|clause| clause.lits.iter()
            .map(|lit| {
                let mul = if lit.negated { -1 } else { 1 };
                (match var_to_ind_map.get(&lit.var) {
                    Some(&ind) => ind,
                    None => {
                        var_to_ind_map.insert(lit.var.clone(),
                                              var_to_ind_map.len() + 1);
                        ind_to_var_map.insert(var_to_ind_map.len() as i32,
                                              lit.var.clone());
                        var_to_ind_map.len()
                    },
                } as i32) * mul
            })
            .collect()
        )
        .collect();

    (Cnf::new(clauses, var_to_ind_map.len()), ind_to_var_map)
}

fn solution_to_js_obj(solution: &Result<HashSet<Lit>, ()>,
                      var_map: &HashMap<i32, String>) -> js_sys::Object {
    let sat_key = JsValue::from("sat");
    let assignments_key = JsValue::from("assignments");

    let js_obj_map = js_sys::Map::new();
    match solution {
        Ok(assign) => {
            let assign_js_array = assignments_to_js_array(&assign, var_map);

            js_obj_map.set(&sat_key, &JsValue::TRUE);
            js_obj_map.set(&assignments_key, &assign_js_array);
        }
        Err(()) => {
            js_obj_map.set(&sat_key, &JsValue::FALSE);
        }
    }

    js_sys::Object::from_entries(&JsValue::from(js_obj_map)).unwrap()
}

fn assignments_to_js_array(assign: &HashSet<Lit>,
                           var_map: &HashMap<i32, String>) -> js_sys::Array {
    let assign_js_array = js_sys::Array::new();

    assign.iter()
        .filter(|lit| lit > &&0)
        .map(|lit| var_map.get(lit).unwrap())
        .map(|var_name| JsValue::from(var_name))
        .for_each(|var_name| {
            assign_js_array.push(&var_name);
        });

    assign_js_array
}