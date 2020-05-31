use std::env;
use std::fs;
mod error;

fn main() -> Result<(), error::GeneratorError> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 2 {

        return Err(error::GeneratorError::CliArgumentNoError { expected: 1, found: (args.len() - 1) as u32, }.into());
    }

    let filename = &args[1];
    println!("Filename {}", filename);
    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(error) => return Err(error::GeneratorError::ReadError {source: error}.into()),
    };
    println!("File contents:\n{:?}", contents);
    Ok(())
}
