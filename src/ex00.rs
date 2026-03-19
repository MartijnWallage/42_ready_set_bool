pub fn adder(a: u32, b: u32) -> u32 {
    match b {
        0 => a,
        _ => adder(a ^ b, (a & b) << 1),
    }
}

