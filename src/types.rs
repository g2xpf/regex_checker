#[derive(Debug)]
pub enum RegToken {
    Eps,
    LeftParen,
    RightParen,
    Vert,
    Star,
    Plus,
    Question,
    Dot,
    Char(char),
}
pub type RegTokens = Vec<RegToken>;
