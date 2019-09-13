use std::convert::TryFrom;

#[derive(PartialEq, Eq, Copy, Clone)]
enum Brace {
    Paren,
    Bracket,
    Curly
}

enum Side {
    Left(Brace),
    Right(Brace),
}

impl TryFrom<char> for Side {
    type Error = char;
    fn try_from(value: char) -> Result<Side, Self::Error> {
        use Brace::*;
        use Side::*;
        match value {
            '{' => Ok(Left(Curly)),
            '}' => Ok(Right(Curly)),
            '[' => Ok(Left(Bracket)),
            ']' => Ok(Right(Bracket)),
            '(' => Ok(Left(Paren)),
            ')' => Ok(Right(Paren)),
            _ => Err(value),
        }
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut v: Vec<Brace> = Vec::new();
    for c in string.chars() {
        match Side::try_from(c) {
            Ok(Side::Left(b)) => {
                v.push(b);
            }
            Ok(Side::Right(b)) => {
                match v.pop() {
                    None => return false,
                    Some(b2) => {
                        if b != b2 {
                            return false;
                        }
                    }
                }
            }
            Err(_) => {}
        }
    }

    v.is_empty()
}
