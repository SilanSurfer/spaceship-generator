use multimap::MultiMap;
use std::env;
use std::fs;

mod error;
mod spaceship;
use error::SpaceshipError;
use spaceship::Spaceship;

fn parse_filename(filename: &str) -> Result<MultiMap<String, String>, SpaceshipError> {
    println!("Reading from file {}", filename);

    let mut spaceship_parts = MultiMap::new();
    let contents = fs::read_to_string(filename)
        .map_err(|filename| SpaceshipError::UnableToReadFile(filename.to_string()))?;
    for iter in contents.lines() {
        let mut elems: Vec<&str> = iter.split_whitespace().collect();
        let key = elems
            .pop()
            .expect("Something went wrong and it shouldn't")
            .to_string();
        let value = elems.join(" ");
        spaceship_parts.insert(key, value);
    }
    Ok(spaceship_parts)
}

fn main() -> Result<(), SpaceshipError> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 2 {
        return Err(SpaceshipError::WrongNumberOfArguments(args.len() as u16));
    }

    let parts = parse_filename(&args[1])?;
    let spaceship = Spaceship::generate_from_dict(&parts);
    println!(
        "Generated spaceship:\n{}",
        spaceship.expect("Something went wrong while generating ship!!!")
    );
    Ok(())
}
