use std::iter::Peekable;

#[derive(Debug, PartialEq)]
enum Node {
    Char(char),
    Seq(Vec<Node>),
    Star(Box<Node>),
    Or((Box<Node>, Box<Node>)),
}

fn _parse(input_it: &mut Peekable<impl Iterator<Item = char>>) -> Option<Node> {
    let c = input_it.next()?;

    let n = Node::Char(c);

    match input_it.peek() {
        Some(c) => {
            if *c == '*' {
                input_it.next();
                return Some(Node::Star(Box::new(n)));
            } else if *c == '|' {
                input_it.next();
                let rhs = _parse(input_it)?;
                return Some(Node::Or((Box::new(n), Box::new(rhs))));
            }
        }
        None => return Some(n),
    }

    let mut seq = vec![n];
    while let Some(next) = _parse(input_it) {
        match next {
            Node::Seq(mut cont) => seq.append(&mut cont),
            _ => seq.push(next),
        }
    }

    return Some(Node::Seq(seq));
}

fn parse(input: &str) -> Node {
    if let Some(n) = _parse(&mut input.chars().peekable()) {
        return n;
    }

    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_char() {
        assert_eq!(parse("a"), Node::Char('a'));
    }

    #[test]
    fn test_parse_seq() {
        assert_eq!(
            parse("ab"),
            Node::Seq(vec![Node::Char('a'), Node::Char('b')])
        );
        assert_eq!(
            parse("abc"),
            Node::Seq(vec![Node::Char('a'), Node::Char('b'), Node::Char('c')])
        );
    }

    #[test]
    fn test_parse_star() {
        assert_eq!(parse("a*"), Node::Star(Box::new(Node::Char('a'))));
        assert_eq!(
            parse("abc*"),
            Node::Seq(vec![
                Node::Char('a'),
                Node::Char('b'),
                Node::Star(Box::new(Node::Char('c')))
            ])
        );
    }

    #[test]
    fn test_parse_or() {
        assert_eq!(
            parse("a|b"),
            Node::Or((Box::new(Node::Char('a')), Box::new(Node::Char('b'))))
        );
        assert_eq!(
            parse("a|b|c"),
            Node::Or((
                Box::new(Node::Char('a')),
                Box::new(Node::Or((
                    Box::new(Node::Char('b')),
                    Box::new(Node::Char('c'))
                )))
            ))
        );
    }
}
