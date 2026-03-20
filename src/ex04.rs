use std::collections::BTreeSet;
use std::collections::HashMap;

fn eval(formula: &str, vars: &HashMap<char, bool>) -> bool {
    let mut stack: Vec<bool> = Vec::new();

    for c in formula.chars() {
        let result = match c {
            '0' | '1'               => c == '1',
            '!'                     => !stack.pop().unwrap(),
            c if c.is_uppercase()   => *vars.get(&c).unwrap(),
            op                      => {
                let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
                match op {
                    '&' => a & b,
                    '|' => a | b,
                    '^' => a ^ b,
                    '>' => !a | b,
                    '=' => a == b,
                    _   => unreachable!("Invalid operator: {op}."),
                }
            }
        };
        stack.push(result);
    }
    stack.pop().unwrap()
}

pub fn print_truth_table(formula: &str) {

    // First collect all the variables
    let vars: Vec<char> = formula
        .chars()
        .filter(|c| c.is_uppercase())
        .collect::<BTreeSet<_>>() // deduplicate and sort
        .into_iter()
        .collect();

    let n = vars.len();

    for v in &vars {
        print!("| {v} ");
    }
    println!("| = |");

    for _ in 0..=n {
        print!("|---");
    }
    println!("|");

    // One row per combination
    for i in 0..(1u32 << n) {
       let assignments: HashMap<char, bool> = vars
           .iter()
           .enumerate()
           .map(|(j, &var)| {
               let val = (i >> (n - 1 - j)) & 1 == 1;
               (var, val)
           })
           .collect();

        for &var in &vars {
            print!("| {} ", assignments[&var] as u8);
        }

        let result = eval(formula, &assignments);
        println!("| {} |", result as u8);
    }
}
