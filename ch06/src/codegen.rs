use crate::parse::Node;

#[derive(PartialEq, Debug)]
struct Inst {
    op: Op,
}

#[derive(PartialEq, Debug)]
enum Op {
    Char(char),
    Jmp(usize),
    Split(usize, usize),
    Match,
}

struct Generator {
    line: usize,
}

impl Generator {
    pub fn new() -> Self {
        Generator { line: 0 }
    }

    pub fn gen(&mut self, n: &Node) -> Vec<Inst> {
        let mut insts = self.gen_expr(n);
        insts.push(Inst { op: Op::Match });
        insts
    }

    fn gen_expr(&mut self, n: &Node) -> Vec<Inst> {
        match n {
            Node::Char(c) => {
                self.line += 1;
                vec![Inst { op: Op::Char(*c) }]
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
                insts.push(Inst {
                    op: Op::Split(l + 1, l + lhs.len() + 2),
                });

                insts.append(&mut lhs);

                let l = self.line;
                self.line += 1; // for Jmp

                let mut rhs = self.gen_expr(rhs);

                insts.push(Inst {
                    op: Op::Jmp(l + rhs.len() + 1),
                });

                insts.append(&mut rhs);

                insts
            }
            Node::Star(n) => {
                let l = self.line;
                self.line += 1; // for Split

                let mut lhs = self.gen_expr(n);

                let mut insts = vec![];
                insts.push(Inst {
                    op: Op::Split(l + 1, l + lhs.len() + 2),
                });

                insts.append(&mut lhs);

                insts.push(Inst { op: Op::Jmp(l) });
                self.line += 1;

                insts
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(_in: Node, _out: Vec<Inst>) {
        let mut g = Generator::new();
        assert_eq!(g.gen(&_in), _out);
    }

    #[test]
    fn test_codegen_char() {
        test(
            Node::Char('a'),
            vec![Inst { op: Op::Char('a') }, Inst { op: Op::Match }],
        )
    }

    #[test]
    fn test_codegen_seq() {
        test(
            Node::Seq(vec![Node::Char('a'), Node::Char('b'), Node::Char('c')]),
            vec![
                Inst { op: Op::Char('a') },
                Inst { op: Op::Char('b') },
                Inst { op: Op::Char('c') },
                Inst { op: Op::Match },
            ],
        )
    }

    #[test]
    fn test_codegen_or() {
        test(
            Node::Or((Box::new(Node::Char('a')), Box::new(Node::Char('b')))),
            vec![
                Inst {
                    op: Op::Split(1, 3),
                },
                Inst { op: Op::Char('a') },
                Inst { op: Op::Jmp(4) },
                Inst { op: Op::Char('b') },
                Inst { op: Op::Match },
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
                Inst {
                    op: Op::Split(1, 3),
                },
                Inst { op: Op::Char('a') },
                Inst { op: Op::Jmp(7) },
                Inst {
                    op: Op::Split(4, 6),
                },
                Inst { op: Op::Char('b') },
                Inst { op: Op::Jmp(7) },
                Inst { op: Op::Char('c') },
                Inst { op: Op::Match },
            ],
        );
    }

    #[test]
    fn test_codegen_star() {
        test(
            Node::Star(Box::new(Node::Char('a'))),
            vec![
                Inst {
                    op: Op::Split(1, 3),
                },
                Inst { op: Op::Char('a') },
                Inst { op: Op::Jmp(0) },
                Inst { op: Op::Match },
            ],
        );
    }
}
