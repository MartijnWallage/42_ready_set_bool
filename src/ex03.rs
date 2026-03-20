pub fn eval_formula(formula: &str) -> bool {
    let mut stack: Vec<bool> = Vec::new();

    for c in formula.chars() {
        let result = match c {
            '0' | '1'   => c == '1',
            '!'         => !stack.pop().unwrap(),
            op          => {
                let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
                match op {
                    '&' => a & b,
                    '|' => a | b,
                    '^' => a ^ b,
                    '>' => !a | b,
                    '=' => a == b,
                    _   => unreachable!("Not a valid operator: {op}."),
                }
            }
        };
        stack.push(result);
    }
    stack.pop().unwrap()
}
