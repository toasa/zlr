use crate::codegen::Inst;

pub fn eval(insts: Vec<Inst>, target: &str) -> bool {
    let mut rm = Machine {
        pc: 0,
        sp: 0,
        insts: insts,
        target_str: target.chars().collect(),
    };
    rm.run()
}

struct Machine {
    pc: usize, // program counter
    sp: usize, // string pointer

    insts: Vec<Inst>,
    target_str: Vec<char>,
}

impl Machine {
    fn run(&mut self) -> bool {
        while self.pc < self.insts.len() {
            match self.insts[self.pc] {
                Inst::Match => return true,
                Inst::Char(c) => {
                    if c == self.target_str[self.sp] {
                        self.pc += 1;
                        self.sp += 1
                    } else {
                        return false;
                    }
                }
                Inst::Jmp(n) => self.pc = n,
                Inst::Split(pc1, pc2) => {
                    self.pc = pc1;
                    let _sp = self.sp;
                    if self.run() {
                        return true;
                    }

                    self.pc = pc2;
                    self.sp = _sp;

                    return self.run();
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::gen;
    use crate::parse::parse;

    fn test(regex: &str, test_str: &str) {
        let n = parse(regex);
        let insts = gen(n);
        assert!(eval(insts, test_str));
    }

    #[test]
    fn test_eval_char() {
        test("a", "a");
    }

    #[test]
    fn test_eval_seq() {
        test("abc", "abc");
    }

    #[test]
    fn test_eval_or() {
        test("a|c", "a");
    }
}
