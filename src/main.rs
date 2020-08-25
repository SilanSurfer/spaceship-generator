use multimap::MultiMap;
use std::env;
use std::fs;

mod spaceship;
use spaceship::error::SpaceshipError;
use spaceship::spaceship::Spaceship;

fn parse_filename(filename: &String) -> MultiMap<String, String> {
    println!("Reading from file {}", filename);

    let mut spaceship_parts = MultiMap::new();
    let contents = fs::read_to_string(filename).unwrap();
    for iter in contents.lines() {
        let mut elems: Vec<&str> = iter.split_whitespace().collect();
        let key = elems
            .pop()
            .expect("Something went wrong and it shouldn't")
            .to_string();
        let value = elems.join(" ");
        spaceship_parts.insert(key, value);
    }
    spaceship_parts
}

fn main() -> Result<(), SpaceshipError> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 2 {
        return Err(SpaceshipError::WrongNumberOfArguments(args.len() as u16));
    }

    let parts = parse_filename(&args[1]);
    let spaceship = Spaceship::generate_from_dict(&parts);
    println!(
        "Generated spaceship:\n{}",
        spaceship.expect("Something went wrong while generating ship!!!")
    );
    Ok(())
}
