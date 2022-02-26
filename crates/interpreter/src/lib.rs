use lexer::*;

// todo: return exit code
pub fn execute_script(script: String) {
    print_tokens(&script);
}

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