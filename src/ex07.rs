use crate::ex06::conjunctive_normal_form;
use std::collections::{HashMap, HashSet};

type Literal        = (char, bool);
type Assignment     = HashMap<Literal>;
type Disjunction    = Vec<Literal>;
type Conjunction    = Vec<Disjunction>;


fn parse(formula: String) -> Conjunction {
    let mut literals: Disjunction = Vec::new();
    let mut clauses: Conjunction = Vec::new();

    for ch in formula {
        match ch {
            '!' => {
                (var, _) = literals.pop().expect("Expected operand for negation.");
                literals.push((var, false));
            },
            '&' => { 
                let right = literals.pop().expect("Expected operand for conjunction.");
                let left = literals.pop().expect("Expected operand for conjunction.");
                let clause = Vec::from([left, right]);
                clauses.push(clause);
            },
            '|' => { 
                let right = literals.pop().expect("Expected operand for disjunction.");
                let left = literals.pop().expect("Expected operand for disjunction.");
                let clause = Vec::from([left, right]);
                clauses.push(clause);
            },
            _   => literals.push((ch, true)),
        }
    }
    clauses
}

fn find_unit(formula: &Conjunction) -> Option<Literal> {
    for clause in formula {
        if clause.len() == 1 {
            return Some(clause[0])
        }
    }
    None
}

fn simplify(formula: &mut Conjunction, assign: &Assignment) -> Option<Conjunction> {
    for clause in formula {
        for (var, pol) in clause {
            match assign.get(var) {
                true => clause.pop(),
                false => // drop 
            }
        }
    }
}

fn find_pure(formula: &Conjunction) -> Vec<Literal> {
    let mut pos: HashSet<char> = HashSet::new();
    let mut neg: HashSet<char> = HashSet::new();

    for clause in formula {
        for &(var, pol) in clause {
            if pol { 
                pos.insert(var);
            } else { 
                neg.insert(var);
            }
        }
    }

    let mut pure: Vec<Literal> = Vec::new();

    for &var in &pos {
        if !neg.contains(var) {
            pure.push((var, true));
        }
    }

    for &var in &neg {
        if !pos.contains(var) {
            pure.push((var, false));
        }
    }
    pure
}

fn pick_variable(formula: &Conjunction, assign: &Assignment) -> Option<char> {
    for clause in formula {
        for &(var, _) in clause { return Some(var) }
    }
    None
}

fn dpll(formula: mut Conjunction, assign: mut Assignment) -> Option<Assign> {
    // unit propagate
    loop {
        match find_unit(&formula) {
            None        => break,
            (var, pol)  => {
                assign.insert(var, pol);
                formula = simplify(&formula, &assign)?;
            }
        }
    }

    // pure literal assign
    let pures = find_pure(&formula);
    for (var, pol) in pures {
        assign.insert(var, pol);
        formula = simplify(&formula, &assign)?;
    }

    // If formula is empty then it is satisfied
    if formula.is_empty() { return Some(assign) }

    // No need to check for empty clauses because
    // we have returned None in simplify

    let var = pick_variable(&formula)?;

    let true_assign = assign.clone().insert(var, true);
    if let result = dpll(formula, true_assign) { return result }
    
    assign.insert(var, false);
    dpll(formula, assign)
}

pub fn sat(formula: &str) -> bool {
    let cnf = conjunctive_normal_form(formula);
    let mut conjunction = parse(cnf);
    match dpll(conjunction) {
        Some(_) => true,
        None    => false,
    }
}
