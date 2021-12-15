use crate::logos_token::BasicToken;
use logos::Logos;
use std::ops::Range;

pub mod logos_token;
pub mod token;
pub mod token_collector;

pub fn print_tokens(script: &String) {
    let tokens: Vec<_> = initial_scan(script);

    for token in tokens {
        println!("{:?}", token.0);
    }
}

pub fn initial_scan(script: &String) -> Vec<(BasicToken, Range<usize>)> {
    // keep the ranges for future error handling
    let tokens: Vec<_> = BasicToken::lexer(script).spanned().collect();
    tokens
}

// pub fn iterate_to_mid(tokens: Vec<BasicToken>) -> Vec<MidLevelToken> {
//     let mut vec = Vec<MidLevelToken>::new();
//     let mut iter = tokens.iter().peekable();
//     while (iter.next() != None) {

//     }
// }
