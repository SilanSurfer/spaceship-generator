use multimap::MultiMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 2 {
        eprintln!(
            "Wrong number of arguments, should be one found {}",
            args.len() - 1
        );
        return;
    }

    let filename = &args[1];
    println!("Filename {}", filename);

    let mut spaceship_parts = MultiMap::new();

    let contents = fs::read_to_string(filename).unwrap();
    for iter in contents.lines() {
        let mut elems: Vec<&str> = iter.split_whitespace().collect();
        // TODO: think about error handling and what should be done if element doesn't met requirements
        let key = elems.pop().expect("Something went wrong and it shouldn't");
        let value = elems.join(" ");
        spaceship_parts.insert(key, value);
    }
    println!("{:?}", spaceship_parts);
}
