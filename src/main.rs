mod ex00;
mod ex01;
mod ex02;
mod ex03;
mod ex04;
mod ex05;
mod ex06;

fn main() {
    // Exercise 00 test:
    println!("\n\nTesting exercise 00\n");
    let cases = [(3, 5), (2, 6), (100, 102), (300, 10), (0,0), (0,1)];
    for (a, b) in cases {
        println!("{a} + {b} = {}", ex00::adder(a, b));
    }

    // Exercise 01 test
    println!("\n\nTesting exercise 01\n");
    let cases = [(0,1),(1,0),(1,1),(0,0),(2,3),(101,5),(2,2),(100,102)];
    for (a,b) in cases {
        println!("{a} * {b} = {}", ex01::multiplier(a, b));
    }

    // Exercise 02 test
    println!("\n\nTesting exercise 02\n");
    for i in 0..16 {
        println!("The gray code of {i:>2} is {:>8b}", ex02::gray_code(i));
    }

    // Exercise 03 test
    println!("\n\nTesting exercise 03\n");
    let cases = ["1!", "0!", "10!&", "10&", "10|", "10|1&", "101|&", "10>", "01>", "110!^>", "10&11!&="];
    for formula in cases {
        println!("{formula} is {}", ex03::eval_formula(&formula));
    }

    // Exercise 04 test
    println!("\n\nTesting exercise 04\n");
    let cases = ["AB&", "AB|", "DJ>", "ABC|&"];
    for case in cases {
        println!("\n{case}");
        ex04::print_truth_table(case);
    }

    // Exercise 05 test
    println!("\n\nTesting exercise 05\n");
    let cases = ["A", "AB&", "AB|", "AB^", "AB>", "AB=", "A!", "A!!", "AB&!", 
        "AB&!!", "A!B&", "A!B&!", "AB!&", "AB!&!", "AB^!", "AB!^!", "AB>!", 
        "AB=!", "AB&C|!", "AB|C&!"];
    for case in cases {
        println!("Testing {case}");
        println!("{}", ex05::negation_normal_form(case));
    }

    // CNF test
    println!("\n\nTesting exercise 06: CNF conversion\n");
    let cases = [
        ("A",           "A"),
        ("A!",          "A!"),
        ("A!!",         "A"),
        ("AB&",         "AB&"),
        ("AB|",         "AB|"),
        ("AB&C|",       "AC|BC|&"),
        ("CAB&|",       "CA|CB|&"),
        ("AB|C&",       "AB|C&"),
        ("AB|CD|&",     "AB|CD|&"),
        ("AB&CD&|",     "AC|AD|&BC|&BD|&"),
        ("AB&!",        "A!B!|"),
        ("AB|!",        "A!B!&"),
        ("A!B!&!",      "AB|"),
        ("A!B&",        "A!B&"),
        ("AB!&",        "AB!&"),
        ("AB&C|D&",     "AC|BC|&D&"),
        ("AB&C|D&E|",       "ACE||BCE||&DE|&"),
        ("AB&C&DE&|",       "AD|AE|&BD|&BE|&CD|&CE|&"),
        ("AB&CD&|EF&|",     "ACE||ACF||&ADE||&ADF||&BCE||&BCF||&BDE||&BDF||&"),
        ("AB|!CD|!|",       "A!C!|A!D!|&B!C!|&B!D!|&"),
        ("AB&C|!",          "A!B!|C!&"),
        ("AB^",         "AB|A!B!|&"),
        ("AB>",         "A!B|"),
        ("AB=",         "A!B|AB!|&"),
        ("AB^C|",       "AB|C|A!B!|C|&"),
        ("AB>C&",       "A!B|C&"),
        ("AB>C>",       "AC|B!C|&"),
        ("AB=C|",       "A!B|C|AB!|C|&"),
        ("AB^!",        "A!B|AB!|&"),
        ("AB^CD^&",     "AB|A!B!|&CD|&C!D!|&"),
        ("AB=C=",       "AB|C|A!B!|C|&A!B|C!|&AB!|C!|&"),
    ];
//    for (case, expected) in cases {
//        let result = ex06::conjunctive_normal_form(case);
//        if result == expected {
//            println!("✓ {case} => {result}");
//        } else {
//            println!("✗ {case} => {result} (expected {expected})");
//        }
//    }

    for (input, expected_cnf) in cases {
        let result = ex06::conjunctive_normal_form(input);
        assert!(
            are_equivalent(input, &result),
            "Output '{}' is not semantically equivalent to input '{}'",
            result, input
        );
        assert!(
            are_equivalent(&result, expected_cnf),
            "Output '{}' doesn't match expected CNF '{}' for input '{}'",
            result, expected_cnf, input
        );
    }
}

fn rpn_eval(formula: &str, assignment: &std::collections::HashMap<char, bool>) -> bool {
    let mut stack: Vec<bool> = Vec::new();

    for c in formula.chars() {
        match c {
            'A'..='Z' => stack.push(*assignment.get(&c).unwrap_or(&false)),
            '!' => {
                let a = stack.pop().unwrap();
                stack.push(!a);
            }
            '&' => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a && b);
            }
            '|' => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a || b);
            }
            '^' => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a ^ b);
            }
            '>' => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(!a || b);  // A→B = ¬A∨B
            }
            '=' => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a == b);   // A↔B
            }
            _ => {}
        }
    }
    stack.pop().unwrap()
}

fn are_equivalent(f1: &str, f2: &str) -> bool {
    // Collect all variables
    let vars: std::collections::HashSet<char> = f1.chars()
        .chain(f2.chars())
        .filter(|c| c.is_uppercase())
        .collect();
    let vars: Vec<char> = vars.into_iter().collect();

    // Try all 2^n assignments
    for mask in 0..(1u32 << vars.len()) {
        let assignment: std::collections::HashMap<char, bool> = vars.iter()
            .enumerate()
            .map(|(i, &v)| (v, (mask >> i) & 1 == 1))
            .collect();

        if rpn_eval(f1, &assignment) != rpn_eval(f2, &assignment) {
            return false;
        }
    }
    true
}

#[test]
fn test_semantic_equivalence() {
    let cases = vec![
        ("A", "A"),
        ("AB&C|", "AC|BC|&"),
        ("AB&!", "A!B!|"),
        ("AB|!", "A!B!&"),
        ("AB&CD&|", "AC|AD|&BC|&BD|&"),
    ];

}
