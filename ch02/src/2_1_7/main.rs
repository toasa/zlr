fn print_upper_string(s: &str) {
    println!("{}", s.to_uppercase());
}

fn main() {
    let s = "This was a triumph!".to_string();
    print_upper_string(&s); // &String から &str へは自動的に型変換される：

    let s1 = r##"こんにちは
こんばんわ"##;
    println!("{}", s1);
}
