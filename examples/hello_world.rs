fn main() {
    let script = "let string example = \"test\"
// testing!
if (example == \"test\") {
    println(\"Hello world!\")
} else {
    println(\"Uh oh...\")
}
";
    lem_lang::print_tokens(&script.to_string());
}