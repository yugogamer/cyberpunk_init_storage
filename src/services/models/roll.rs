use juniper::GraphQLObject;
use rand::Rng;
use serde::{Deserialize, Serialize};

use super::character::Character;

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject)]
pub struct CharacterRoll {
    pub character: Character,
    pub roll: Roll,
}

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject)]
pub struct Roll {
    pub roll: i32,
    pub critical_roll: Option<i32>,
    pub critical: bool,
    pub total: i32,
}

pub fn roll_initiative(character: &Vec<Character>) -> Vec<CharacterRoll> {
    let mut characters_rolls: Vec<CharacterRoll> = Vec::new();
    for c in character {
        let roll = roll_dice(c);
        let character_roll = CharacterRoll {
            character: c.clone(),
            roll,
        };
        characters_rolls.push(character_roll);
    }
    characters_rolls
}

fn roll_dice(character: &Character) -> Roll {
    let mut res = Roll {
        roll: 0,
        critical_roll: None,
        critical: false,
        total: 0,
    };

    let mut rng = rand::thread_rng();
    let roll: i32 = rng.gen_range(1..=10);
    res.roll = roll;
    if roll == 10 {
        let critical_roll = rng.gen_range(1..=10);
        res.critical_roll = Some(critical_roll);
        res.critical = true;
    }
    res.total = res.roll + character.base_ref + character.modifier + res.critical_roll.unwrap_or(0);
    res
}
