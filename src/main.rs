use multimap::MultiMap;
use std::env;
use std::fs;

mod spaceship;
use spaceship::spaceship::Spaceship;
use spaceship::error::SpaceshipError;

fn main() -> Result<(), SpaceshipError>{
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 2 {
        return Err(SpaceshipError::WrongNumberOfArguments(args.len() as u16));
    }

    let filename = &args[1];
    println!("Reading from file {}", filename);

    let mut spaceship_parts = MultiMap::new();

    let contents = fs::read_to_string(filename).unwrap();
    for iter in contents.lines() {
        let mut elems: Vec<&str> = iter.split_whitespace().collect();
        // TODO: think about error handling and what should be done if element doesn't met requirements
        let key = elems.pop().expect("Something went wrong and it shouldn't");
        let value = elems.join(" ");
        spaceship_parts.insert(key, value);
    }
    let spaceship = Spaceship::generate_from_file(&spaceship_parts);
    println!("Generated spaceship:\n{}", spaceship.expect("Something went wrong while generating ship!!!"));
    Ok(())
}
