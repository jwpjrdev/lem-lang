pub use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum BasicToken {
    // basic keywords
    #[token("nameof")]
    KeywordNameof,
    #[token("is")]
    KeywordIs,
    #[token("true")]
    KeywordTrue,
    #[token("false")]
    KeywordFalse,
    #[token("int")]
    KeywordInt,
    #[token("string")]
    KeywordString,
    #[token("boolean")]
    KeywordBoolean,
    #[token("throw")]
    KeywordThrow,
    #[token("let")]
    KeywordLet,
    #[token("if")]
    KeywordIf,
    #[token("else")]
    KeywordElse,
    #[token("for")]
    KeywordFor,
    #[token("in")]
    KeywordIn,
    #[token("while")]
    KeywordWhile,
    #[token("return")]
    KeywordReturn,
    #[token("fn")]
    KeywordFunction,

    // symbols
    #[token("(")]
    SymbolLeftParen,
    #[token(")")]
    SymbolRightParen,
    #[token("[")]
    SymbolLeftSquareBracket,
    #[token("]")]
    SymbolRightSquareBracket,
    #[token("{")]
    SymbolLeftCurlyBracket,
    #[token("}")]
    SymbolRightCurlyBracket,
    #[token("=")]
    SymbolVarAssign,
    #[token("+")]
    SymbolPlus,
    #[token("-")]
    SymbolMinus,
    #[token("*")]
    SymbolStar,
    #[token("/")]
    SymbolSlash,
    #[token("%")]
    SymbolModulo,
    #[token("!")]
    SymbolBang,
    #[token("^")]
    SymbolPower,
    #[token(",")]
    SymbolComma,
    #[token(".")]
    SymbolDot,
    
    // comparators
    #[token("&&")]
    ComparatorAnd,
    #[token("||")]
    ComparatorOr,
    #[token("==")]
    ComparatorEqual,
    #[token("!=")]
    ComparatorNotEqual,
    #[token(">")]
    ComparatorGreater,
    #[token("<")]
    ComparatorLess,
    #[token(">=")]
    ComparatorGreaterEqual,
    #[token("<=")]
    ComparatorLessEqual,

    // numbers & strings
    #[regex(r"(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?")]
    Number,
    #[regex(r"(?:0|[1-9][0-9]*)\.[^0-9]")]
    ErrorNumJunkAfterDecimalPoint,
    #[regex(r"(?:0|[1-9][0-9]*)(?:\.[0-9]+)?[eE][^+\-0-9]")]
    ErrorNumJunkAfterExponent,
    #[regex(r"(?:0|[1-9][0-9]*)(?:\.[0-9]+)?[eE][+-][^0-9]")]
    ErrorNumJunkAfterExponentSign,

    #[regex("\"(?s:[^\"\\\\]|\\\\.)*\"")]
    String,

    // other
    #[regex(r"[_a-zA-Z][_a-zA-Z0-9]*")]
    Identifier,
    #[regex(r"//[^\r\n]*(\r\n|\n)?")]
    SingleLineSlashComment,
    #[error]
    #[regex(r"[ \t\n\f\r]+", logos::skip)]
    Error,
}
