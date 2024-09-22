use crate::parse::Node;

#[derive(PartialEq, Debug)]
struct Inst {
    op: Op,
    line: u32,
}

#[derive(PartialEq, Debug)]
enum Op {
    Char(char),
    Jmp(u32),
    Split(u32, u32),
    Match,
}

struct Generator {
    line: u32,
    insts: Vec<Inst>,
}

impl Generator {
    pub fn gen(&mut self, n: &Node) {
        self.gen_expr(n);
        self.insts.push(Inst {
            op: Op::Match,
            line: self.line,
        });
    }

    fn gen_expr(&mut self, n: &Node) {
        match n {
            Node::Char(c) => {
                self.insts.push(Inst {
                    op: Op::Char(*c),
                    line: self.line,
                });
                self.line += 1;
            }
            Node::Seq(seq) => {
                for e in seq {
                    self.gen_expr(e);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(_in: Node, _out: Vec<Inst>) {
        let mut g = Generator {
            line: 0,
            insts: vec![],
        };
        g.gen(&_in);
        assert_eq!(g.insts, _out);
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
}
