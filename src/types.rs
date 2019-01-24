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

#[derive(Debug)]
pub enum RegExp {
    EmptyExp,
    EpsExp,
    CharExp(char),
    ConcatExp(Box<RegExp>, Box<RegExp>),
    AltExp(Box<RegExp>, Box<RegExp>),
    QuestionExp,
    StarExp,
    PlusExp,
    DotExp,
}
pub type RegExps = Vec<RegExp>;
