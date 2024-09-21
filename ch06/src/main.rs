use std::iter::Peekable;

#[derive(Debug, PartialEq)]
enum Node {
    Char(char),
    Seq(Vec<Node>),
}

fn _parse(mut input_it: Peekable<impl Iterator<Item = char>>) -> Option<Node> {
    let c = input_it.next()?;

    let n = Node::Char(c);

    if input_it.peek() == None {
        return Some(n);
    }

    let mut seq = vec![n];
    while let Some(next) = input_it.next() {
        seq.push(Node::Char(next));
    }

    return Some(Node::Seq(seq));
}

fn parse(input: &str) -> Node {
    if let Some(n) = _parse(input.chars().peekable()) {
        return n;
    }

    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        assert_eq!(parse("a"), Node::Char('a'));
        assert_eq!(
            parse("abc"),
            Node::Seq(vec![Node::Char('a'), Node::Char('b'), Node::Char('c')])
        );
    }
}
