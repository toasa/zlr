mod codegen;
mod parse;

fn main() {
    let node = parse::parse("abc");
    _ = codegen::gen(&node);
}
