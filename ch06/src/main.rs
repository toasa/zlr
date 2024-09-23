mod codegen;
mod evaluator;
mod parse;

fn main() {
    let node = parse::parse("abc");
    let insts = codegen::gen(node);
    evaluator::eval(insts, "abc");
}
