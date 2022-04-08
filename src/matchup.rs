use crate::Player;
use crate::Position;
use crate::Team;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum MatchupKind {
    TwoWayScrim,
    ThreeWayScrim,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Matchup {
    pub kind: MatchupKind,
    pub teams: Vec<(String, u8, u8, u8, u8, u8, u8)>,
    pub sr_average: f32,
    pub sr_diff: f32,
    pub rating: i16,
}

impl Matchup {
    pub fn new(
        teams: Vec<(String, u8, u8, u8, u8, u8, u8)>,
        players: &HashMap<u8, Player>,
    ) -> Matchup {
        let kind = if teams.len() == 2 {
            MatchupKind::TwoWayScrim
        } else {
            MatchupKind::ThreeWayScrim
        };

        let mut created_teams: Vec<Team> = Vec::new();
        for team_comp in &teams {
            let team = Team::new(
                team_comp.0.clone(),
                players[&team_comp.1].clone(),
                players[&team_comp.2].clone(),
                players[&team_comp.3].clone(),
                players[&team_comp.4].clone(),
                players[&team_comp.5].clone(),
                players[&team_comp.6].clone(),
            );
            created_teams.push(team);
        }

        let mut sr_average: f32 = 0.0;
        for team in &created_teams {
            sr_average += team.get_average_sr();
        }
        sr_average /= teams.len() as f32;

        let mut sr_diff: f32 = 0.0;
        for team in &created_teams {
            sr_diff += (team.get_average_sr() - sr_average).abs();
        }

        let mut sum_of_standard_deviations: f32 = 0.0;
        for team in &created_teams {
            sum_of_standard_deviations += team.get_standard_deviation();
        }

        let average_of_standard_deviations: f32 = sum_of_standard_deviations / teams.len() as f32;

        let mut sum_of_deviations_of_standard_deviations: f32 = 0.0;
        for team in &created_teams {
            sum_of_deviations_of_standard_deviations +=
                (team.get_standard_deviation() - average_of_standard_deviations).powf(2.0);
        }

        let rating = (sr_diff.powi(2) + sum_of_deviations_of_standard_deviations) as i16;

        Matchup {
            kind,
            teams,
            sr_average,
            sr_diff,
            rating,
        }
    }

    pub fn _pretty_print(&self, players: &HashMap<u8, Player>) {
        print!("{}", self.get_pretty_string(players));
    }

    pub fn get_extended_string(&self, players: &HashMap<u8, Player>) -> String {
        let mut extended_string = self.get_pretty_string(players);

        let position_vec = vec![Position::Tank, Position::Damage, Position::Support];

        let mut strings: HashMap<Position, String> = HashMap::new();
        let mut averages: HashMap<Position, f32> = HashMap::new();
        let mut full_role_average: HashMap<Position, f32> = HashMap::new();
        let mut standard_deviations: HashMap<Position, f32> = HashMap::new();
        let mut average_deviations: HashMap<Position, f32> = HashMap::new();

        let number_of_teams = self.teams.len() as f32;

        let mut created_teams: Vec<Team> = Vec::new();
        for team in &self.teams {
            let team = Team::new(
                team.0.clone(),
                players[&team.1].clone(),
                players[&team.2].clone(),
                players[&team.3].clone(),
                players[&team.4].clone(),
                players[&team.5].clone(),
                players[&team.6].clone(),
            );
            created_teams.push(team);
        }

        for position in &position_vec {
            strings.insert(position.clone(), String::new());
            averages.insert(position.clone(), 0.0);
            standard_deviations.insert(position.clone(), 0.0);
        }

        for team in &created_teams {
            for position in &position_vec {
                averages.insert(position.clone(), averages[position] + team.get_average_sr_of_role_duo(position.clone()));
                standard_deviations.insert(position.clone(), standard_deviations[position] + team.get_standard_deviation_of_role_duo(position.clone()));
            }
        }

        for position in &position_vec {
            full_role_average.insert(position.clone(), averages[position] / number_of_teams);
            average_deviations.insert(position.clone(), standard_deviations[position] / number_of_teams);
        }

        let mut sum_of_all_avg_diff = 0.0;
        let mut sum_of_all_dev_diff = 0.0;

        for position in &position_vec {
            extended_string.push_str("\n----------------------------------------\n");
            extended_string.push_str(&format!("{:?} Values:\n\n", position));
            extended_string.push_str(&format!("Average SR over all teams: {:.2}\n", full_role_average[position]));
            extended_string.push_str(&format!("Average deviation: {:.2}\n\n", average_deviations[position]));
            extended_string.push_str("Avg  SR ");
            for team in &created_teams {
                extended_string.push_str(&format!("{: >25.1}", team.get_average_sr_of_role_duo(position.clone())));
            }
            extended_string.push_str("\n");
            extended_string.push_str("Avg Diff");
            for team in &created_teams {
                let avg_diff = (team.get_average_sr_of_role_duo(position.clone()) - full_role_average[position]).abs();
                extended_string.push_str(&format!("{: >25.1}", avg_diff));
                sum_of_all_avg_diff += avg_diff;
            }
            extended_string.push_str("\n\n");
            extended_string.push_str("Dev     ");
            for team in &created_teams {
                extended_string.push_str(&format!("{: >25.1}", team.get_standard_deviation_of_role_duo(position.clone())));
            }
            extended_string.push_str("\n");
            extended_string.push_str("Dev Diff");
            for team in &created_teams {
                let dev_diff = (team.get_standard_deviation_of_role_duo(position.clone()) - average_deviations[position]).abs();
                extended_string.push_str(&format!("{: >25.1}", dev_diff));
                sum_of_all_dev_diff += dev_diff;
            }
            extended_string.push_str("\n");
        }
        
        extended_string.push_str("\n----------------------------------------\n");
        extended_string.push_str("Global Values\n\n");
        extended_string.push_str(&format!("Sum of all Average differences:   {:.2}\n", sum_of_all_avg_diff));
        extended_string.push_str(&format!("Sum of all Deviation differences: {:.2}\n", sum_of_all_dev_diff));



        extended_string
    }

    pub fn get_pretty_string(&self, players: &HashMap<u8, Player>) -> String {
        let mut s = String::new();
        s.push_str("-------------------------------------\n");
        s.push_str("Matchup\n");
        s.push_str(&format!("Average SR: {}\n", self.sr_average));
        s.push_str(&format!("Difference to Average: {}\n", self.sr_diff));
        s.push_str(&format!("Rating: {}\n\n", self.rating));

        let mut team_names = String::new();
        let mut tank_line_1 = String::new();
        let mut tank_line_2 = String::new();
        let mut damage_line_1 = String::new();
        let mut damage_line_2 = String::new();
        let mut support_line_1 = String::new();
        let mut support_line_2 = String::new();

        for team in self.teams.clone() {
            team_names.push_str(&format!("{: >25}", &team.0));
            tank_line_1.push_str(
                &format!(
                    "{: >25}",
                    players.get(&team.1).unwrap().print_role(Position::Tank)
                )
                .as_str(),
            );
            tank_line_2.push_str(
                &format!(
                    "{: >25}",
                    players.get(&team.2).unwrap().print_role(Position::Tank)
                )
                .as_str(),
            );
            damage_line_1.push_str(
                &format!(
                    "{: >25}",
                    players.get(&team.3).unwrap().print_role(Position::Damage)
                )
                .as_str(),
            );
            damage_line_2.push_str(
                &format!(
                    "{: >25}",
                    players.get(&team.4).unwrap().print_role(Position::Damage)
                )
                .as_str(),
            );
            support_line_1.push_str(
                &format!(
                    "{: >25}",
                    players.get(&team.5).unwrap().print_role(Position::Support)
                )
                .as_str(),
            );
            support_line_2.push_str(
                &format!(
                    "{: >25}",
                    players.get(&team.6).unwrap().print_role(Position::Support)
                )
                .as_str(),
            );
        }
        s.push_str(&format!("        {}\n", team_names));
        s.push_str(&format!("        {}\n", tank_line_1));
        s.push_str(&format!("        {}\n", tank_line_2));
        s.push_str(&format!("        {}\n", damage_line_1));
        s.push_str(&format!("        {}\n", damage_line_2));
        s.push_str(&format!("        {}\n", support_line_1));
        s.push_str(&format!("        {}\n", support_line_2));

        // s.push_str("-------------------------------------\n");

        s
    }
}
