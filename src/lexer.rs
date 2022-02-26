use logos::Logos;
use std::ops::Range;

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
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
    // lem doesn't use semicolons but lexing them anyways makes error handling easier
    #[token(";")]
    SymbolSemicolon,

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

pub fn initial_scan(script: &str) -> Vec<(self::BasicToken, Range<usize>)> {
    // keep the ranges for future error handling
    let tokens: Vec<_> = self::BasicToken::lexer(script).spanned().collect();
    tokens
}

pub fn strip_token_vec(result: &Vec<(self::BasicToken, Range<usize>)>) -> Vec<self::BasicToken> {
    let mut stripped_vec: Vec<self::BasicToken> = Vec::new();
    for (token, _) in result {
        stripped_vec.push(*token);
    }
    stripped_vec
}

#[cfg(test)]
mod tests {
    use crate::lexer::BasicToken::*;

    #[test]
    fn test_lexer() {
        let tokens = "nameof is true false int string boolean throw let if else for in while return fn
    ( ) [ ] { } = + - * / % ! ^ , . ;
    && || == != > < >= <= 10 10.1 \"string\" identifier
    // comment";
        let result = crate::lexer::initial_scan(tokens);
        assert_eq!(crate::lexer::strip_token_vec(&result), vec! [
            KeywordNameof,
            KeywordIs,
            KeywordTrue,
            KeywordFalse,
            KeywordInt,
            KeywordString,
            KeywordBoolean,
            KeywordThrow,
            KeywordLet,
            KeywordIf,
            KeywordElse,
            KeywordFor,
            KeywordIn,
            KeywordWhile,
            KeywordReturn,
            KeywordFunction,
            SymbolLeftParen,
            SymbolRightParen,
            SymbolLeftSquareBracket,
            SymbolRightSquareBracket,
            SymbolLeftCurlyBracket,
            SymbolRightCurlyBracket,
            SymbolVarAssign,
            SymbolPlus,
            SymbolMinus,
            SymbolStar,
            SymbolSlash,
            SymbolModulo,
            SymbolBang,
            SymbolPower,
            SymbolComma,
            SymbolDot,
            SymbolSemicolon,
            ComparatorAnd,
            ComparatorOr,
            ComparatorEqual,
            ComparatorNotEqual,
            ComparatorGreater,
            ComparatorLess,
            ComparatorGreaterEqual,
            ComparatorLessEqual,
            Number,
            Number,
            String,
            Identifier,
            SingleLineSlashComment,
        ]);
    }

    #[test]
    fn test_strip_token_vec() {
        let lexed = crate::lexer::initial_scan("nameof");
        let stripped = crate::lexer::strip_token_vec(&lexed);
        assert_eq!(lexed.get(0).unwrap().0, *stripped.get(0).unwrap());
    }
}
