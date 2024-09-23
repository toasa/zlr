use crate::codegen::Inst;

pub fn eval(insts: Vec<Inst>, target: &str) -> bool {
    let mut rm = Machine { pc: 0, sp: 0 };
    rm.run(insts, target)
}

struct Machine {
    pc: usize, // program counter
    sp: usize, // string pointer
}

impl Machine {
    fn run(&mut self, insts: Vec<Inst>, target: &str) -> bool {
        let chars: Vec<char> = target.chars().collect();

        while self.pc < insts.len() {
            match insts[self.pc] {
                Inst::Match => return true,
                Inst::Char(c) => {
                    if c == chars[self.sp] {
                        self.pc += 1;
                        self.sp += 1
                    } else {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_char() {
        assert!(eval(vec![Inst::Char('a'), Inst::Match], "a"));
    }
}
