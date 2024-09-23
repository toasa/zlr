use crate::codegen::Inst;

pub fn eval(insts: Vec<Inst>, target: &str) -> bool {
    let mut rm = Machine {
        pc: 0,
        sp: 0,
        insts,
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
                    if self.sp >= self.target_str.len() {
                        return false;
                    }

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

    fn test(regex: &str, test_str: &str, match_: bool) {
        let n = parse(regex);
        let insts = gen(n);
        assert_eq!(eval(insts, test_str), match_);
    }

    fn match_(regex: &str, test_str: &str) {
        test(regex, test_str, true);
    }

    fn not_match(regex: &str, test_str: &str) {
        test(regex, test_str, false);
    }

    #[test]
    fn test_eval_char() {
        match_("a", "a");
        not_match("a", "b");
    }

    #[test]
    fn test_eval_seq() {
        match_("abc", "abc");
        match_("abc", "abcd");

        not_match("abc", "a");
        not_match("abc", "abd");
        not_match("abc", "abdc");
    }

    #[test]
    fn test_eval_or() {
        match_("a|b", "a");
        match_("a|b", "b");

        not_match("a|b", "c");
    }

    #[test]
    fn test_eval_star() {
        match_("a*", "a");
        match_("a*", "aaaa");
    }

    #[test]
    fn test_eval_composite() {
        match_("a(bc|de)", "abc");
        match_("a(bc|de)", "ade");

        match_("abc|def", "abc");
        match_("abc|def", "def");

        match_("a(bc)*d", "ad");
        match_("a(bc)*d", "abcd");
        match_("a(bc)*d", "abcbcbcd");
    }
}
