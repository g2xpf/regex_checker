use crate::types::*;

pub struct Lexer {
    tokens: RegTokens,
}

impl Lexer {
    pub fn new(regex: &str) -> Self {
        Lexer {
            tokens: Lexer::parse(regex),
        }
    }

    pub fn tokens(&self) -> &RegTokens {
        &self.tokens
    }

    pub fn tokens_mut(&mut self) -> &mut RegTokens {
        &mut self.tokens
    }

    fn parse(regex: &str) -> RegTokens {
        let mut v = vec![];
        let mut iter = regex.chars().into_iter();

        while let Some(c) = iter.next() {
            match c {
                'ε' => v.push(RegToken::Eps),
                '(' => v.push(RegToken::LeftParen),
                ')' => v.push(RegToken::RightParen),
                '|' => v.push(RegToken::Vert),
                '*' => v.push(RegToken::Star),
                '+' => v.push(RegToken::Plus),
                '?' => v.push(RegToken::Question),
                '.' => v.push(RegToken::Dot),
                _ => v.push(RegToken::Char(c)),
            }
        }
        v
    }
}
