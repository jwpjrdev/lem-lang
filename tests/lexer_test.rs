use lem_lang::logos_token::BasicToken::*;

#[test]
fn test_lexer() {
    let tokens = "nameof is true false int string boolean throw let if else for in while return fn
( ) [ ] { } = + - * / % ! ^ , .
&& || == != > < >= <= 10 10.1 \"string\" identifier
// comment";
    let result = lem_lang::initial_scan(&tokens.to_string());
    assert_eq!(lem_lang::strip_token_vec(&result), vec! [
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
