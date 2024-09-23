use std::{iter::Peekable, vec};

#[derive(Debug, PartialEq)]
pub enum Node {
    Char(char),
    Seq(Vec<Node>),
    Star(Box<Node>),
    Or((Box<Node>, Box<Node>)),
}

// Operator's precedence is followings(from high to low):
//   1. ()
//   2. *, +, ?
//   3.  (Concatenation)
//   4. |
//
// Reference is here:
//   https://pubs.opengroup.org/onlinepubs/9799919799/nframe.html
//
// or     = concat ("|" or)?
// concat = star (star)*
// star   = group "*"?
// group  = '(' or ')'
//        | 'a'..'z'
pub fn parse(input: &str) -> Node {
    let mut chars = input.chars().peekable();
    parse_or(&mut chars)
}

fn parse_or<I>(chars: &mut Peekable<I>) -> Node
where
    I: Iterator<Item = char>,
{
    let mut n = parse_concat(chars);
    if let Some(c) = chars.peek() {
        if *c == '|' {
            chars.next();
            n = Node::Or((Box::new(n), Box::new(parse_or(chars))))
        }
    }

    n
}

fn parse_concat<I>(chars: &mut Peekable<I>) -> Node
where
    I: Iterator<Item = char>,
{
    let mut seq = vec![];
    while let Some(&c) = chars.peek() {
        match c {
            ')' | '|' => break,
            _ => seq.push(parse_star(chars)),
        }
    }

    if seq.len() == 1 {
        seq.pop().unwrap()
    } else {
        Node::Seq(seq)
    }
}

fn parse_star<I>(chars: &mut Peekable<I>) -> Node
where
    I: Iterator<Item = char>,
{
    let mut n = parse_group(chars);
    if let Some(&c) = chars.peek() {
        if c == '*' {
            chars.next();
            n = Node::Star(Box::new(n));
        }
    }

    n
}

fn parse_group<I>(chars: &mut Peekable<I>) -> Node
where
    I: Iterator<Item = char>,
{
    let c = chars.next().unwrap();
    match c {
        '(' => {
            let n = parse_or(chars);
            assert_eq!(chars.next().unwrap(), ')');
            n
        }
        _ => Node::Char(c),
    }
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
        assert_eq!(
            parse("a|bc*"),
            Node::Or((
                Box::new(Node::Char('a')),
                Box::new(Node::Seq(vec![
                    Node::Char('b'),
                    Node::Star(Box::new(Node::Char('c')))
                ]))
            ))
        );

        assert_eq!(
            parse("abc|def"),
            Node::Or((
                Box::new(Node::Seq(vec![
                    Node::Char('a'),
                    Node::Char('b'),
                    Node::Char('c')
                ])),
                Box::new(Node::Seq(vec![
                    Node::Char('d'),
                    Node::Char('e'),
                    Node::Char('f')
                ]))
            )),
        );
    }
}
