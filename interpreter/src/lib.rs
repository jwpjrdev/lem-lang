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