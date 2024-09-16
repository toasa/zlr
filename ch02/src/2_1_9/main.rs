fn do_(f: fn(u32, u32) -> u32, a: u32, b: u32) {
    println!("{}", f(a, b));
}

fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn mul(a: u32, b: u32) -> u32 {
    a * b
}

fn main() {
    do_(add, 11, 22);
    do_(mul, 11, 22);
}
