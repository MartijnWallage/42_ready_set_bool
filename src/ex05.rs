#[derive(Clone, Debug)]
enum Expr {
    Var(char),
    Not(Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn not(expr: Expr) -> Expr {
        Expr::Not(Box::new(expr))
    }

    fn and(left: Expr, right: Expr) -> Expr {
        Expr::And(Box::new(left), Box::new(right))
    }

    fn or(left: Expr, right: Expr) -> Expr {
        Expr::Or(Box::new(left), Box::new(right))
    }
}

fn parse_formula(formula: &str) -> Expr {
    let mut stack: Vec<Expr> = Vec::new();

    for ch in formula.chars() {
        match ch {
            '!'     => {
                let operand = stack.pop().expect("Expected operand for negation.");
                stack.push(Expr::not(operand));
            },
            '&'     => {
                let right = stack.pop().expect("Expected right operand for conjunction.");
                let left = stack.pop().expect("Expected left operand for conjunction");
                stack.push(Expr::and(left, right));
            },
            '|'     => {
                let right = stack.pop().expect("Expected right operand for disjunction.");
                let left = stack.pop().expect("Expected left operand for disjunction.");
                stack.push(Expr::or(left, right));
            },
            '^'     => {
                let right = stack.pop().expect("Expected right operand for xor.");
                let left = stack.pop().expect("Expected left operand or xor.");
                stack.push(Expr::or(
                        Expr::and(left.clone(), Expr::not(right.clone())), 
                        Expr::and(Expr::not(left), right)
                        ));
            },
            '='     => {
                let right = stack.pop().expect("Expected right operand for iff.");
                let left = stack.pop().expect("Expected left operand for iff.");
                stack.push(Expr::or(
                        Expr::and(left.clone(), right.clone()),
                        Expr::and(Expr::not(left), Expr::not(right))
                        ));
            },
            '>'     => {
                let right = stack.pop().expect("Expected right operand for >.");
                let left = stack.pop().expect("Expected left operand for >.");
                stack.push(Expr::or(Expr::not(left), right));
            }
            _       => stack.push(Expr::Var(ch))
        }
    }

    assert_eq!(stack.len(), 1, "Invalid formula.");
    stack.pop().unwrap()
}

fn to_nnf(expr: Expr) -> Expr {
    match expr {
        Expr::Var(c)            => Expr::Var(c),
        Expr::And(left, right)  => Expr::and(to_nnf(*left), to_nnf(*right)),
        Expr::Or(left, right)   => Expr::or(to_nnf(*left), to_nnf(*right)),
        Expr::Not(operand)      => match *operand {
            Expr::Var(c)            => Expr::not(Expr::Var(c)),
            Expr::Not(inner)        => to_nnf(*inner),
            Expr::And(left, right)  => {
                let new_left = to_nnf(Expr::not(*left));
                let new_right = to_nnf(Expr::not(*right));
                Expr::or(new_left, new_right)
            }
            Expr::Or(left, right)   => {
                let new_left = to_nnf(Expr::not(*left));
                let new_right = to_nnf(Expr::not(*right));
                Expr::and(new_left, new_right)
            }
        }
    }
}

fn to_rpn(expr: &Expr) -> String {
    match expr {
        Expr::Var(c)    => c.to_string(),
        Expr::Not(operand) => {
            let mut result = to_rpn(operand);
            result.push('!');
            result
        },
        Expr::And(left, right) => {
            let mut result = to_rpn(left);
            result.push_str(&to_rpn(right));
            result.push('&');
            result
        },
        Expr::Or(left, right) => {
            let mut result = to_rpn(left);
            result.push_str(&to_rpn(right));
            result.push('|');
            result
        }
    }
}

pub fn negation_normal_form(formula: &str) -> String {
    let expr = parse_formula(formula); // Parse rpn as tree
    let nnf = to_nnf(expr);    // Recursively apply De Morgan Laws
    to_rpn(&nnf)               // Convert back to rpn
}
