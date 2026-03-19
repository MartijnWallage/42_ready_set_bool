mod ex00;
mod ex01;
mod ex02;

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

    println!("\n\nTesting exercise 02\n");
    for i in 0..16 {
        println!("The gray code of {i:>2} is {:>8b}", ex02::gray_code(i));
    }
}

