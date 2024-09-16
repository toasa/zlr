fn main() {
    let x = 10;
    let y = 20;
    println!("mul({}, {}) = {}", x, y, mul(x, y));
}

fn mul(x: i32, y: i32) -> i32 {
    x * y
}
