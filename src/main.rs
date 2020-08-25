use multimap::MultiMap;
use std::env;
use std::fs;
use std::fmt;
use rand::prelude::*;

mod error;

use error::Error;

#[derive(Debug)]
struct Spaceship {
    engine: String,
    fuselage: String,
    cabin: String,
    small_wings: Option<String>,
    big_wings: Option<String>,
    armor: String,
    weapons: Vec<String>,
}

impl Spaceship {
    fn generate_from_file(avail_parts: &MultiMap<&str, String>) -> Result<Spaceship, Error> {
        let engine_parts = avail_parts.get_vec("engine").ok_or_else(|| Error::LackOfPartInTheFile("engine".to_string()))?;
        let fuselage_parts = avail_parts.get_vec("fuselage").ok_or_else(|| Error::LackOfPartInTheFile("fuselage".to_string()))?;
        let cabin_parts = avail_parts.get_vec("cabin").ok_or_else(|| Error::LackOfPartInTheFile("cabin".to_string()))?;
        let armor_parts = avail_parts.get_vec("armor").ok_or_else(|| Error::LackOfPartInTheFile("armor".to_string()))?;
        let wings_parts = avail_parts.get_vec("wings").ok_or_else(|| Error::LackOfPartInTheFile("wings".to_string()))?;
        let weapon_parts = avail_parts.get_vec("weapon").ok_or_else(|| Error::LackOfPartInTheFile("weapon".to_string()))?;
        Ok(Spaceship {
            engine: Spaceship::draw_element_from(engine_parts),
            fuselage: Spaceship::draw_element_from(fuselage_parts),
            cabin: Spaceship::draw_element_from(cabin_parts),
            small_wings: if random() {
                None
            } else {
                Some(Spaceship::draw_element_from(wings_parts))
            },
            big_wings: if random() {
                None
            } else {
                Some(Spaceship::draw_element_from(wings_parts))
            },
            armor: Spaceship::draw_element_from(armor_parts),
            weapons: Spaceship::draw_multiple_weapons(weapon_parts),
        })
    }

    fn draw_element_from(avail_parts: &Vec<String>) -> String {
        let mut rng = thread_rng();
        let element = rng.gen_range(0, avail_parts.len());
        avail_parts[element].to_string()
    }

    fn draw_multiple_weapons(avail_parts: &Vec<String>) -> Vec<String> {
        let mut weapons = Vec::new();
        let mut rng = thread_rng();
        let no = rng.gen_range(0, 5);
        for _ in 0..no {
            weapons.push(avail_parts[rng.gen_range(0, avail_parts.len())].to_string());
        }
        weapons
    }
}

impl fmt::Display for Spaceship {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "A ship with:")?;
        writeln!(f, "{} engine,", self.engine)?;
        writeln!(f, "{} fuselage,", self.fuselage)?;
        writeln!(f, "{} cabin,", self.cabin)?;
        if let Some(small_wings) = &self.small_wings {
            writeln!(f, "{} wings,", small_wings)?;
        }
        if let Some(big_wings) = &self.big_wings {
            writeln!(f, "{} wings,", big_wings)?;
        }
        writeln!(f, "{} armor,", self.armor)?;
        writeln!(f, "weapons:")?;
        if self.weapons.is_empty() {
            writeln!(f, "None")?;
        } else {
            for weapon in &self.weapons {
                writeln!(f, "{},", weapon)?;
            }
        }
        Ok(())
    }
}

fn main() -> Result<(), Error>{
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 2 {
        return Err(Error::WrongNumberOfArguments(args.len() as u16));
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
