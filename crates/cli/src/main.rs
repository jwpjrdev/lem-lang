use clap::{command, Command, arg};
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    // lem path/to/script.lem
    let matches = command!()
        .name("lem-cli")
        .arg_required_else_help(true)
        .arg(
            arg!([script])
                .takes_value(true)
                .allow_invalid_utf8(true)
            )
        .get_matches();
    
    if let Some(raw_path) = matches.value_of_os("script") {
        let path = Path::new(raw_path);
        if path.exists() {
            if path.is_file() {
                // for now, require the .lem file extension to run
                if let Some(extension) = path.extension() {
                    if extension.to_str().unwrap() == String::from("lem") {
                        println!("Good to go!");
                        println!("  {}", path.display());
                        let script = std::fs::read_to_string(path)?;
                        // this doesn't check for things like \n or \r
                        if !script.trim().is_empty() {
                            interpreter::execute_script(script);
                        } else {
                            eprintln!("The provided script file is empty");
                        }
                    } else {
                        eprintln!("The provided file does not have the .lem extension:");
                        eprintln!("  {}", path.display());
                    }
                } else {
                    eprintln!("The provided file has no extension:");
                    eprintln!("  {}", path.display());
                }
            } else {
                eprintln!("The provided path is a directory:");
                eprintln!("  {}", path.display());
            }
        } else {
            eprintln!("The provided path does not exist:");
            eprintln!("  {}", path.display());
        }
    }
    
    Ok(())
}
