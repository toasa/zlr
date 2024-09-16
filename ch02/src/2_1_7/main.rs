fn print_upper_string(s: &str) {
    println!("{}", s.to_uppercase());
}

fn main() {
    let a = "This was a triumph!".to_string();
    print_upper_string(&a); // &String から &str へは自動的に型変換される：
}
