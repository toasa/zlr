use crate::parse::Node;

#[derive(PartialEq, Debug)]
pub enum Inst {
    Char(char),
    Jmp(usize),
    Split(usize, usize),
    Match,
}

pub fn gen(n: &Node) -> Vec<Inst> {
    let mut g = Generator { line: 0 };
    let mut insts = g.gen_expr(n);
    insts.push(Inst::Match);
    insts
}

struct Generator {
    line: usize,
}

impl Generator {
    fn gen_expr(&mut self, n: &Node) -> Vec<Inst> {
        match n {
            Node::Char(c) => {
                self.line += 1;
                vec![Inst::Char(*c)]
            }
            Node::Seq(seq) => {
                let mut insts = vec![];
                for e in seq {
                    insts.append(&mut self.gen_expr(e));
                }
                insts
            }
            Node::Or((lhs, rhs)) => {
                let l = self.line;
                self.line += 1; // for Split

                let mut lhs = self.gen_expr(lhs);

                let mut insts = vec![];
                insts.push(Inst::Split(l + 1, l + lhs.len() + 2));

                insts.append(&mut lhs);

                let l = self.line;
                self.line += 1; // for Jmp

                let mut rhs = self.gen_expr(rhs);

                insts.push(Inst::Jmp(l + rhs.len() + 1));

                insts.append(&mut rhs);

                insts
            }
            Node::Star(n) => {
                let l = self.line;
                self.line += 1; // for Split

                let mut lhs = self.gen_expr(n);

                let mut insts = vec![];
                insts.push(Inst::Split(l + 1, l + lhs.len() + 2));

                insts.append(&mut lhs);

                insts.push(Inst::Jmp(l));
                self.line += 1;

                insts
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(_in: Node, expect: Vec<Inst>) {
        assert_eq!(gen(&_in), expect);
    }

    #[test]
    fn test_codegen_char() {
        test(Node::Char('a'), vec![Inst::Char('a'), Inst::Match])
    }

    #[test]
    fn test_codegen_seq() {
        test(
            Node::Seq(vec![Node::Char('a'), Node::Char('b'), Node::Char('c')]),
            vec![
                Inst::Char('a'),
                Inst::Char('b'),
                Inst::Char('c'),
                Inst::Match,
            ],
        )
    }

    #[test]
    fn test_codegen_or() {
        test(
            Node::Or((Box::new(Node::Char('a')), Box::new(Node::Char('b')))),
            vec![
                Inst::Split(1, 3),
                Inst::Char('a'),
                Inst::Jmp(4),
                Inst::Char('b'),
                Inst::Match,
            ],
        );

        test(
            Node::Or((
                Box::new(Node::Char('a')),
                Box::new(Node::Or((
                    Box::new(Node::Char('b')),
                    Box::new(Node::Char('c')),
                ))),
            )),
            vec![
                Inst::Split(1, 3),
                Inst::Char('a'),
                Inst::Jmp(7),
                Inst::Split(4, 6),
                Inst::Char('b'),
                Inst::Jmp(7),
                Inst::Char('c'),
                Inst::Match,
            ],
        );
    }

    #[test]
    fn test_codegen_star() {
        test(
            Node::Star(Box::new(Node::Char('a'))),
            vec![
                Inst::Split(1, 3),
                Inst::Char('a'),
                Inst::Jmp(0),
                Inst::Match,
            ],
        );
    }
}
