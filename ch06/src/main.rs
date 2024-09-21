use std::{iter::Peekable, vec};

#[derive(Debug, PartialEq)]
enum Node {
    Char(char),
    Seq(Vec<Node>),
    Star(Box<Node>),
    Or((Box<Node>, Box<Node>)),
}

fn parse(input: &str) -> Node {
    let mut chars = input.chars().peekable();
    parse_expr(&mut chars)
}

fn parse_expr<I>(chars: &mut Peekable<I>) -> Node
where
    I: Iterator<Item = char>,
{
    let mut seq = vec![];
    while let Some(&c) = chars.peek() {
        match c {
            ')' => break,
            _ => seq.push(parse_atom(chars)),
        }
    }

    if seq.len() == 1 {
        seq.pop().unwrap()
    } else {
        Node::Seq(seq)
    }
}

fn parse_atom<I>(chars: &mut Peekable<I>) -> Node
where
    I: Iterator<Item = char>,
{
    let mut node = match chars.next().unwrap() {
        '(' => {
            let node = parse_expr(chars);
            assert_eq!(chars.next(), Some(')'));
            node
        }
        c => Node::Char(c),
    };

    if let Some(&next) = chars.peek() {
        match next {
            '*' => {
                chars.next();
                node = Node::Star(Box::new(node));
            }
            '|' => {
                chars.next();
                let right = parse_expr(chars);
                node = Node::Or((Box::new(node), Box::new(right)));
            }
            _ => {}
        }
    }

    node
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

    #[test]
    fn test_parse_composite() {
        assert_eq!(parse("(a)"), Node::Char('a'));
        assert_eq!(
            parse("(ab)"),
            Node::Seq(vec![Node::Char('a'), Node::Char('b')])
        );
        assert_eq!(
            parse("(abc)"),
            Node::Seq(vec![Node::Char('a'), Node::Char('b'), Node::Char('c')])
        );
        assert_eq!(
            parse("(ab)*"),
            Node::Star(Box::new(Node::Seq(vec![Node::Char('a'), Node::Char('b')])))
        );
        assert_eq!(
            parse("ab(c|de)"),
            Node::Seq(vec![
                Node::Char('a'),
                Node::Char('b'),
                Node::Or((
                    Box::new(Node::Char('c')),
                    Box::new(Node::Seq(vec![Node::Char('d'), Node::Char('e')])),
                ))
            ])
        );
        assert_eq!(
            parse("a(bc)*d"),
            Node::Seq(vec![
                Node::Char('a'),
                Node::Star(Box::new(Node::Seq(vec![Node::Char('b'), Node::Char('c')]))),
                Node::Char('d'),
            ])
        );
    }
}
