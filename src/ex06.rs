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

fn rpn_to_tree(formula: &str) -> Expr {
    let mut stack: Vec<Expr> = Vec::new();

    for ch in formula.chars() {
        match ch {
            '!' => {
                let operand = stack.pop().expect("Expected operand for negation.");
                stack.push(Expr::not(operand));
            },
            '&' => {
                let right = stack.pop().expect("Expected right operand for conjunction.");
                let left = stack.pop().expect("Expected left operand for conjunction.");
                stack.push(Expr::and(left, right));
            },
            '|' => {
                let right = stack.pop().expect("Expected right operand for disjunction.");
                let left = stack.pop().expect("Expected left operand for disjunction.");
                stack.push(Expr::or(left, right));
            },
            '^' => {
                let right = stack.pop().expect("Expected right operand for xor.");
                let left = stack.pop().expect("Expected left operand for xor.");
                stack.push(Expr::and(
                        Expr::or(left.clone(), right.clone()),
                        Expr::or(Expr::not(left), Expr::not(right))
                        ));
            },
            '>' => {
                let right = stack.pop().expect("Expected right operand for implication.");
                let left = stack.pop().expect("Expected left operand for implication.");
                stack.push(Expr::or(Expr::not(left), right));
            },
            '=' =>  {
                let right = stack.pop().expect("Expected right operand for iff.");
                let left = stack.pop().expect("Expected left operand for iff.");
                stack.push(Expr::and(
                        Expr::or(left.clone(), Expr::not(right.clone())),
                        Expr::or(Expr::not(left), right)
                ));
            },
            _   =>  {
                stack.push(Expr::Var(ch));
            }
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
            Expr::Var(ch)           => Expr::not(Expr::Var(ch)),
            Expr::Not(inner)        => to_nnf(*inner),
            Expr::And(left, right)  => Expr::or(to_nnf(Expr::not(*left)), to_nnf(Expr::not(*right))),
            Expr::Or(left, right)   => Expr::and(to_nnf(Expr::not(*left)), to_nnf(Expr::not(*right)))
        }
    }
}

fn distribute(expr: Expr) -> Expr {
    match expr {
        Expr::Var(c)            => Expr::Var(c),
        Expr::Not(operand)      => Expr::not(distribute(*operand)),
        Expr::And(left, right)  => Expr::and(distribute(*left), distribute(*right)),
        Expr::Or(a, b)          => match (*a, *b) {
            (Expr::And(aa, ab), _)  => Expr::and(distribute(Expr::or(*aa, *b.clone())), distribute(Expr::or(*ab, *b))),
            (_, Expr::And(ba, bb))  => Expr::and(distribute(Expr::or(*a.clone(), *ba)), distribute(Expr::or(*a, *bb))),
            _                       => Expr::or(*a, *b)
        }
    }
}

fn to_rpn(expr: Expr) -> String {
    match expr {
        Expr::Var(c)            => c.to_string(),
        Expr::Not(operand)      => {
            let mut result = to_rpn(*operand);
            result.push('!');
            result
        },
        Expr::And(left, right)  => {
            let mut result = to_rpn(*left);
            result.push_str(&to_rpn(*right));
            result.push('&');
            result
        }
        Expr::Or(left, right)   => {
            let mut result = to_rpn(*left);
            result.push_str(&to_rpn(*right));
            result.push('|');
            result
        }
    }
}

pub fn conjunctive_normal_form(formula: &str) -> String {
    let tree = rpn_to_tree(formula);
    let nnf = to_nnf(tree);
    let cnf = distribute(nnf);
    to_rpn(cnf)
}
