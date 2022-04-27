use crate::Player;
use crate::Position;

// struct OW2Team with parameters name, tank_1, tank_2, damage_1, damage_2, support_1, support_2

#[derive(Debug, Clone, PartialEq)]
pub struct OW2Team {
    pub name: String,
    pub tank: Option<Player>,
    pub damage_1: Option<Player>,
    pub damage_2: Option<Player>,
    pub support_1: Option<Player>,
    pub support_2: Option<Player>,
}

impl OW2Team {
    pub fn new(
        name: String,
        tank: Player,
        damage_1: Player,
        damage_2: Player,
        support_1: Player,
        support_2: Player,
    ) -> OW2Team {
        OW2Team {
            name,
            tank: Some(tank),
            damage_1: Some(damage_1),
            damage_2: Some(damage_2),
            support_1: Some(support_1),
            support_2: Some(support_2),
        }
    }

    pub fn get_average_sr_of_role_duo(&self, position: Position) -> f32 {
        let mut average_sr: i16 = 0;
        match position {
            Position::Tank => {
                if let Some(tank) = &self.tank {
                    average_sr += tank.get_sr(Position::Tank);
                }
            }
            Position::Damage => {
                if let Some(damage_1) = &self.damage_1 {
                    average_sr += damage_1.get_sr(Position::Damage);
                }
                if let Some(damage_2) = &self.damage_2 {
                    average_sr += damage_2.get_sr(Position::Damage);
                }
            }
            Position::Support => {
                if let Some(support_1) = &self.support_1 {
                    average_sr += support_1.get_sr(Position::Support);
                }
                if let Some(support_2) = &self.support_2 {
                    average_sr += support_2.get_sr(Position::Support);
                }
            }
        }
        if position == Position::Tank {
            average_sr as f32
        } else {
            average_sr as f32 / 2.0
        }
    }

    pub fn get_standard_deviation_of_role_duo(&self, position: Position) -> f32 {
        let avg_of_role = self.get_average_sr_of_role_duo(position);
        let mut standard_deviation: f32 = 0.0;

        match position {
            Position::Tank => {
                if let Some(tank) = &self.tank {
                    standard_deviation +=
                        (tank.get_sr(Position::Tank) as f32 - avg_of_role).powf(2.0);
                }
            }
            Position::Damage => {
                if let Some(damage_1) = &self.damage_1 {
                    standard_deviation +=
                        (damage_1.get_sr(Position::Damage) as f32 - avg_of_role).powf(2.0);
                }
                if let Some(damage_2) = &self.damage_2 {
                    standard_deviation +=
                        (damage_2.get_sr(Position::Damage) as f32 - avg_of_role).powf(2.0);
                }
            }
            Position::Support => {
                if let Some(support_1) = &self.support_1 {
                    standard_deviation +=
                        (support_1.get_sr(Position::Support) as f32 - avg_of_role).powf(2.0);
                }
                if let Some(support_2) = &self.support_2 {
                    standard_deviation +=
                        (support_2.get_sr(Position::Support) as f32 - avg_of_role).powf(2.0);
                }
            }
        }

        (standard_deviation as f32).sqrt()
    }

    pub fn get_average_sr(&self) -> f32 {
        let average_sr: f32 = (self.get_all_player_srs().iter().sum::<i16>() as f32) / 5.0;
        average_sr
    }

    pub fn get_standard_deviation(&self) -> f32 {
        let average_sr = self.get_average_sr() as i32;
        let mut standard_deviation: i32 = 0;
        for player in self.get_all_player_srs() {
            standard_deviation += (player as i32 - average_sr as i32).pow(2);
        }
        (standard_deviation as f32).sqrt()
    }

    pub fn get_all_player_srs(&self) -> Vec<i16> {
        let mut all_player_srs: Vec<i16> = Vec::new();
        all_player_srs.push(self.tank.as_ref().unwrap().get_sr(Position::Tank));
        all_player_srs.push(self.damage_1.as_ref().unwrap().get_sr(Position::Damage));
        all_player_srs.push(self.damage_2.as_ref().unwrap().get_sr(Position::Damage));
        all_player_srs.push(self.support_1.as_ref().unwrap().get_sr(Position::Support));
        all_player_srs.push(self.support_2.as_ref().unwrap().get_sr(Position::Support));
        all_player_srs
    }

    pub fn _get_all_players(&self) -> Vec<Player> {
        let mut all_players: Vec<Player> = Vec::new();
        all_players.push(self.tank.as_ref().unwrap().clone());
        all_players.push(self.damage_1.as_ref().unwrap().clone());
        all_players.push(self.damage_2.as_ref().unwrap().clone());
        all_players.push(self.support_1.as_ref().unwrap().clone());
        all_players.push(self.support_2.as_ref().unwrap().clone());
        all_players
    }

    pub fn _to_full_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("OW2Team {}\n", &self.name));
        s.push_str(&format!("Average SR {}\n", &self.get_average_sr()));
        s.push_str(&format!(
            "Standard deviation {}\n",
            &self.get_standard_deviation()
        ));
        s.push_str(&format!(
            "Tank:    {} - {}\n",
            &self
                .tank
                .as_ref()
                .unwrap_or(&Player::create_dummy())
                .get_sr(Position::Tank),
            &self.tank.as_ref().unwrap_or(&Player::create_dummy()).name
        ));
        s.push_str(&format!(
            "Damage:  {} - {}\n",
            &self
                .damage_1
                .as_ref()
                .unwrap_or(&Player::create_dummy())
                .get_sr(Position::Damage),
            &self
                .damage_1
                .as_ref()
                .unwrap_or(&Player::create_dummy())
                .name
        ));
        s.push_str(&format!(
            "Damage:  {} - {}\n",
            &self
                .damage_2
                .as_ref()
                .unwrap_or(&Player::create_dummy())
                .get_sr(Position::Damage),
            &self
                .damage_2
                .as_ref()
                .unwrap_or(&Player::create_dummy())
                .name
        ));
        s.push_str(&format!(
            "Support: {} - {}\n",
            &self
                .support_1
                .as_ref()
                .unwrap_or(&Player::create_dummy())
                .get_sr(Position::Support),
            &self
                .support_1
                .as_ref()
                .unwrap_or(&Player::create_dummy())
                .name
        ));
        s.push_str(&format!(
            "Support: {} - {}\n",
            &self
                .support_2
                .as_ref()
                .unwrap_or(&Player::create_dummy())
                .get_sr(Position::Support),
            &self
                .support_2
                .as_ref()
                .unwrap_or(&Player::create_dummy())
                .name
        ));
        s
    }

    pub fn _get_id(&self) -> String {
        let mut s = String::new();
        // Append tanks to s sorted by name
        s.push_str(self.tank.as_ref().unwrap().name.as_str());
        s.push_str(",");

        // Append damage to s sorted by name
        let mut damage = vec![
            self.damage_1.as_ref().unwrap().clone(),
            self.damage_2.as_ref().unwrap().clone(),
        ];
        damage.sort_by(|a, b| a.name.cmp(&b.name));
        for dam in damage {
            s.push_str(&dam.name);
            s.push_str(",");
        }
        // Append support to s sorted by name
        let mut support = vec![
            self.support_1.as_ref().unwrap().clone(),
            self.support_2.as_ref().unwrap().clone(),
        ];
        support.sort_by(|a, b| a.name.cmp(&b.name));
        for sup in support {
            s.push_str(&sup.name);
            s.push_str(",");
        }
        s
    }
}
