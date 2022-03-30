use crate::Position;
use anyhow::{anyhow, Error, Result};
use std::iter::Iterator;

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub name: String,
    pub tank_sr: i16,
    pub damage_sr: i16,
    pub support_sr: i16,
    pub plays_tank: bool,
    pub plays_damage: bool,
    pub plays_support: bool,
}

impl Player {
    pub fn new(player_string: String) -> Result<Player> {
        let split: Vec<&str> = player_string.split(" - ").collect();
        if split.len() < 5 {
            println!(
                "Splitting Error with the following String: {}",
                player_string
            );
            return Err(anyhow!("Splitting Error"));
        }
        let name = split[0].to_string();
        let tank_sr = split[1].parse::<i16>().unwrap();
        let damage_sr = split[2].parse::<i16>().unwrap();
        let support_sr = split[3].parse::<i16>().unwrap();
        let plays_tank = split[4].to_string().contains("t");
        let plays_damage = split[4].to_string().contains("d");
        let plays_support = split[4].to_string().contains("s");
        let player = Player {
            name,
            tank_sr,
            damage_sr,
            support_sr,
            plays_tank,
            plays_damage,
            plays_support,
        };
        Ok(player)
    }

    pub fn create_dummy() -> Player {
        Player {
            name: "---".to_string(),
            tank_sr: 0,
            damage_sr: 0,
            support_sr: 0,
            plays_tank: false,
            plays_damage: false,
            plays_support: false,
        }
    }

    pub fn get_sr(&self, pos: Position) -> i16 {
        match pos {
            Position::Tank => self.tank_sr,
            Position::Damage => self.damage_sr,
            Position::Support => self.support_sr,
        }
    }

    pub fn plays_position(&self, pos: Position) -> bool {
        match pos {
            Position::Tank => self.plays_tank,
            Position::Damage => self.plays_damage,
            Position::Support => self.plays_support,
        }
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("{}\n", &self.name));
        s.push_str(&format!("plays_tank: {}\n", &self.plays_tank));
        s.push_str(&format!("plays_damage: {}\n", &self.plays_damage));
        s.push_str(&format!("plays_support: {}\n", &self.plays_support));
        s
    }

}
