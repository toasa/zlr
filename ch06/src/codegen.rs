use crate::parse::Node;

#[derive(PartialEq, Debug)]
struct Inst {
    op: Op,
    line: usize,
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
        insts.push(Inst {
            op: Op::Match,
            line: self.line,
        });
        insts
    }

    fn gen_expr(&mut self, n: &Node) -> Vec<Inst> {
        match n {
            Node::Char(c) => {
                let l = self.line;
                self.line += 1;
                vec![Inst {
                    op: Op::Char(*c),
                    line: l,
                }]
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
                let lhs_len = lhs.len();

                let mut insts = vec![];
                insts.push(Inst {
                    op: Op::Split(l + 1, l + lhs_len + 2),
                    line: l,
                });

                insts.append(&mut lhs);

                let l = self.line;
                self.line += 1; // for Jmp

                let mut rhs = self.gen_expr(rhs);
                let rhs_len = rhs.len();

                insts.push(Inst {
                    op: Op::Jmp(l + rhs_len + 1),
                    line: l,
                });

                insts.append(&mut rhs);

                insts
            }
            Node::Star(n) => {
                let l = self.line;
                self.line += 1; // for Split

                let mut lhs = self.gen_expr(n);
                let lhs_len = lhs.len();

                let mut insts = vec![];
                insts.push(Inst {
                    op: Op::Split(l + 1, l + lhs_len + 2),
                    line: l,
                });

                insts.append(&mut lhs);

                insts.push(Inst {
                    op: Op::Jmp(l),
                    line: self.line,
                });
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
            vec![
                Inst {
                    op: Op::Char('a'),
                    line: 0,
                },
                Inst {
                    op: Op::Match,
                    line: 1,
                },
            ],
        )
    }

    #[test]
    fn test_codegen_seq() {
        test(
            Node::Seq(vec![Node::Char('a'), Node::Char('b'), Node::Char('c')]),
            vec![
                Inst {
                    op: Op::Char('a'),
                    line: 0,
                },
                Inst {
                    op: Op::Char('b'),
                    line: 1,
                },
                Inst {
                    op: Op::Char('c'),
                    line: 2,
                },
                Inst {
                    op: Op::Match,
                    line: 3,
                },
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
                    line: 0,
                },
                Inst {
                    op: Op::Char('a'),
                    line: 1,
                },
                Inst {
                    op: Op::Jmp(4),
                    line: 2,
                },
                Inst {
                    op: Op::Char('b'),
                    line: 3,
                },
                Inst {
                    op: Op::Match,
                    line: 4,
                },
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
                    line: 0,
                },
                Inst {
                    op: Op::Char('a'),
                    line: 1,
                },
                Inst {
                    op: Op::Jmp(7),
                    line: 2,
                },
                Inst {
                    op: Op::Split(4, 6),
                    line: 3,
                },
                Inst {
                    op: Op::Char('b'),
                    line: 4,
                },
                Inst {
                    op: Op::Jmp(7),
                    line: 5,
                },
                Inst {
                    op: Op::Char('c'),
                    line: 6,
                },
                Inst {
                    op: Op::Match,
                    line: 7,
                },
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
                    line: 0,
                },
                Inst {
                    op: Op::Char('a'),
                    line: 1,
                },
                Inst {
                    op: Op::Jmp(0),
                    line: 2,
                },
                Inst {
                    op: Op::Match,
                    line: 3,
                },
            ],
        );
    }
}
