
fn eval_dyadic(a: bool, b: bool, op: char) -> bool {
    match op {
        '&' => a&b,
        '|' => a|b,
        '^' => a^b,
        '>' => !a|b,
        '=' => a==b,
        _   => false,  // may ignore
    }
}

pub fn eval_formula(formula: &str) -> bool {
    let mut stack: Vec<bool> = Vec::new();
    for c in formula.chars() {
        match c {
            '0' => stack.push(false),
            '1' => stack.push(true),
            '!' => {
                let a = stack.pop().unwrap();
                stack.push(!a);
            }
            op  => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(eval_dyadic(a, b, op));
            }
        }
    }
    stack.pop().unwrap()
}
