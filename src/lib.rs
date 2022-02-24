use crate::logos_token::BasicToken;
use logos::Logos;
use std::ops::Range;

pub mod logos_token;
pub mod token;
// pub mod token_collector;

pub fn print_tokens(script: &str) {
    let tokens: Vec<_> = initial_scan(script);

    for token in &tokens {
        println!("{:?}", token);
    }

    println!("------------------");

    for token in strip_token_vec(&tokens) {
        println!("{:?}", token);
    }
}

pub fn strip_token_vec(result: &Vec<(BasicToken, Range<usize>)>) -> Vec<BasicToken> {
    let mut stripped_vec: Vec<BasicToken> = Vec::new();
    for (token, _) in result {
        stripped_vec.push(*token);
    }
    stripped_vec
}

pub fn initial_scan(script: &str) -> Vec<(BasicToken, Range<usize>)> {
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
