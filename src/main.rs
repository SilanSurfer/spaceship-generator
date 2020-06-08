use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 2 {
        eprintln!("Wrong number of arguments, should be one found {}", args.len() - 1);
        return;
    }

    let filename = &args[1];
    println!("Filename {}", filename);
    let contents = fs::read_to_string(filename).unwrap();
    for iter in contents.lines() {
        let elems: Vec<&str> = iter.split_whitespace().collect();
        println!("{:?}", elems);
    }
}
