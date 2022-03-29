use crate::Player;
use crate::Position;
use anyhow::Result;

// struct Team with parameters name, tank_1, tank_2, damage_1, damage_2, support_1, support_2

#[derive(Debug, Clone, PartialEq)]
pub struct Team {
    pub name: String,
    pub tank_1: Option<Player>,
    pub tank_2: Option<Player>,
    pub damage_1: Option<Player>,
    pub damage_2: Option<Player>,
    pub support_1: Option<Player>,
    pub support_2: Option<Player>,
}

impl Team {
    pub fn new(
        name: String,
        tank_1: Player,
        tank_2: Player,
        damage_1: Player,
        damage_2: Player,
        support_1: Player,
        support_2: Player,
    ) -> Team {
        Team {
            name,
            tank_1: Some(tank_1),
            tank_2: Some(tank_2),
            damage_1: Some(damage_1),
            damage_2: Some(damage_2),
            support_1: Some(support_1),
            support_2: Some(support_2),
        }
    }

    pub fn get_average_sr(&self) -> Result<f32> {
        let average_sr: f32 = (self.get_all_player_srs().unwrap().iter().sum::<i16>() as f32) / 6.0;
        Ok(average_sr)
    }

    pub fn get_standard_deviation(&self) -> Result<f32> {
        let average_sr = self.get_average_sr()? as i32;
        let mut standard_deviation: i32 = 0;
        for player in self.get_all_player_srs()? {
            standard_deviation += (player as i32 - average_sr as i32).pow(2);
        }
        Ok((standard_deviation as f32).sqrt())
    }

    pub fn get_all_player_srs(&self) -> Result<Vec<i16>> {
        let mut all_player_srs: Vec<i16> = Vec::new();
        all_player_srs.push(self.tank_1.as_ref().unwrap().get_sr(Position::Tank));
        all_player_srs.push(self.tank_2.as_ref().unwrap().get_sr(Position::Tank));
        all_player_srs.push(self.damage_1.as_ref().unwrap().get_sr(Position::Damage));
        all_player_srs.push(self.damage_2.as_ref().unwrap().get_sr(Position::Damage));
        all_player_srs.push(self.support_1.as_ref().unwrap().get_sr(Position::Support));
        all_player_srs.push(self.support_2.as_ref().unwrap().get_sr(Position::Support));
        Ok(all_player_srs)
    }

    pub fn get_all_players(&self) -> Result<Vec<Player>> {
        let mut all_players: Vec<Player> = Vec::new();
        all_players.push(self.tank_1.as_ref().unwrap().clone());
        all_players.push(self.tank_2.as_ref().unwrap().clone());
        all_players.push(self.damage_1.as_ref().unwrap().clone());
        all_players.push(self.damage_2.as_ref().unwrap().clone());
        all_players.push(self.support_1.as_ref().unwrap().clone());
        all_players.push(self.support_2.as_ref().unwrap().clone());
        Ok(all_players)
    }

    pub fn to_full_String(&self) -> Result<String> {
        let mut s = String::new();
        s.push_str(&format!("Team {}\n", &self.name));
        s.push_str(&format!("Average SR {}\n", &self.get_average_sr()?));
        s.push_str(&format!(
            "Standard deviation {}\n",
            &self.get_standard_deviation()?
        ));
        s.push_str(&format!(
            "Tank:    {} - {}\n",
            &self
                .tank_1
                .as_ref()
                .unwrap_or(&Player::create_dummy())
                .get_sr(Position::Tank),
            &self.tank_1.as_ref().unwrap_or(&Player::create_dummy()).name
        ));
        s.push_str(&format!(
            "Tank:    {} - {}\n",
            &self
                .tank_2
                .as_ref()
                .unwrap_or(&Player::create_dummy())
                .get_sr(Position::Tank),
            &self.tank_2.as_ref().unwrap_or(&Player::create_dummy()).name
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
        Ok(s)
    }

    pub fn get_ID(&self) -> Result<String> {
        let mut s = String::new();
        // Append tanks to s sorted by name
        let mut tanks = vec![
            self.tank_1.as_ref().unwrap().clone(),
            self.tank_2.as_ref().unwrap().clone(),
        ];
        tanks.sort_by(|a, b| a.name.cmp(&b.name));
        for tank in tanks {
            s.push_str(&tank.name);
            s.push_str(",");
        }
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
        Ok(s)
    }
}
