fn main() {
    let mut n = 100;

    let a = &mut n;
    *a = 10;

    println!("{}", n);
}
