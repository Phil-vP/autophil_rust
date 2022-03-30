use crate::Team;


#[derive(Debug, Clone, PartialEq)]
pub enum MatchupKind {
    TwoWayScrim,
    ThreeWayScrim
}


#[derive(Debug, Clone, PartialEq)]
pub struct Matchup {
    kind: MatchupKind,
    teams: Vec<Team>,
    sr_average: f32,
    sr_diff: f32,
    rating: i16
}

impl Matchup {
    fn new(teams: Vec<Team>) -> Matchup {
        let kind = if teams.len() == 2 {
            MatchupKind::TwoWayScrim
        } else {
            MatchupKind::ThreeWayScrim
        };

        let mut sr_average: f32 = 0.0;
        for team in teams.clone() {
            sr_average += team.get_average_sr().unwrap();
        }
        sr_average /= teams.len() as f32;

        let mut sr_diff: f32 = 0.0;
        for team in teams.clone() {
            sr_diff += team.get_average_sr().unwrap() - sr_average;
        }
        let rating = 0;

        Matchup {
            kind,
            teams,
            sr_average,
            sr_diff,
            rating
        }
    }

    fn pretty_print(&self) {
        println!("Matchup");
        println!("Kind: {:?}", self.kind);
        println!("Average SR: {}", self.sr_average);
        println!("Difference to Average: {}", self.sr_diff);
        println!("Rating: {}", self.rating);
        println!("\n");
        
        let mut team_names = String::new();
        let mut tank_line_1 = String::new();
        let mut tank_line_2 = String::new();
        let mut damage_line_1 = String::new();
        let mut damage_line_2 = String::new();
        let mut support_line_1 = String::new();
        let mut support_line_2 = String::new();

        for team in self.teams.clone() {
            team_names.push_str(&format!("{: <15}", &team.name));
            tank_line_1.push_str(&format!("{: <15}", [&team.tank_1.clone().unwrap().name, ": ", &team.tank_1.unwrap().tank_sr.to_string()].join("")).as_str());
            tank_line_2.push_str(&format!("{: <15}", [&team.tank_2.clone().unwrap().name, ": ", &team.tank_2.unwrap().tank_sr.to_string()].join("")).as_str());
            damage_line_1.push_str(&format!("{: <15}", [&team.damage_1.clone().unwrap().name, ": ", &team.damage_1.unwrap().damage_sr.to_string()].join("")).as_str());
            damage_line_2.push_str(&format!("{: <15}", [&team.damage_2.clone().unwrap().name, ": ", &team.damage_2.unwrap().damage_sr.to_string()].join("")).as_str());
            support_line_1.push_str(&format!("{: <15}", [&team.support_1.clone().unwrap().name, ": ", &team.support_1.unwrap().support_sr.to_string()].join("")).as_str());
            support_line_2.push_str(&format!("{: <15}", [&team.support_2.clone().unwrap().name, ": ", &team.support_2.unwrap().support_sr.to_string()].join("")).as_str());
        }
    }
}