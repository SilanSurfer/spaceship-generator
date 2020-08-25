use crate::spaceship::error::SpaceshipError;
use multimap::MultiMap;
use rand::prelude::*;
use std::fmt;

#[derive(Debug)]
pub struct Spaceship {
    engine: String,
    fuselage: String,
    cabin: String,
    small_wings: Option<String>,
    big_wings: Option<String>,
    armor: String,
    weapons: Vec<String>,
}

impl Spaceship {
    pub fn generate_from_dict(
        avail_parts: &MultiMap<String, String>,
    ) -> Result<Spaceship, SpaceshipError> {
        let engine_parts = avail_parts
            .get_vec("engine")
            .ok_or_else(|| SpaceshipError::LackOfPartInTheFile("engine".to_string()))?;
        let fuselage_parts = avail_parts
            .get_vec("fuselage")
            .ok_or_else(|| SpaceshipError::LackOfPartInTheFile("fuselage".to_string()))?;
        let cabin_parts = avail_parts
            .get_vec("cabin")
            .ok_or_else(|| SpaceshipError::LackOfPartInTheFile("cabin".to_string()))?;
        let armor_parts = avail_parts
            .get_vec("armor")
            .ok_or_else(|| SpaceshipError::LackOfPartInTheFile("armor".to_string()))?;
        let wings_parts = avail_parts
            .get_vec("wings")
            .ok_or_else(|| SpaceshipError::LackOfPartInTheFile("wings".to_string()))?;
        let weapon_parts = avail_parts
            .get_vec("weapon")
            .ok_or_else(|| SpaceshipError::LackOfPartInTheFile("weapon".to_string()))?;
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
