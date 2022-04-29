use pest::iterators::Pairs;
use lem::parser::*;
use lem::ast::{generate_ast, File};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn parse(script: &String) {
    let parse_tree = LemParser::parse_with_timer(script);
    generate_ast(parse_tree);
}

fn interp(script: &String) {
    lem::execute_script(script)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse+ast hello-world",|b| b.iter(|| parse(&r#"println("Hello, world!");"#.to_string())));
    c.bench_function("parse+ast variable",|b| b.iter(|| parse(&r#"let example = "Hello, world!";"#.to_string())));
    c.bench_function("parse+ast+interp variable-println",|b| b.iter(|| interp(&r#"let example = "Hello, world!"; println(example);"#.to_string())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
