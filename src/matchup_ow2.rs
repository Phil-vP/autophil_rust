use crate::OW2Team;
use crate::Player;
use crate::Position;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum MatchupKind {
    TwoWayScrim,
    ThreeWayScrim,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OW2Matchup {
    pub kind: MatchupKind,
    pub extended_teams: Vec<(String, u8, u8, u8, u8, u8, f32)>,
    pub sr_average: f32,
    pub full_role_average: [f32; 3],
    pub standard_deviations: [f32; 3],
    pub average_deviations: [f32; 3],
    pub rating: i16,
}

impl OW2Matchup {
    pub fn new(
        teams: Vec<(String, u8, u8, u8, u8, u8)>,
        players: &HashMap<u8, Player>,
    ) -> OW2Matchup {
        let kind = if teams.len() == 2 {
            MatchupKind::TwoWayScrim
        } else {
            MatchupKind::ThreeWayScrim
        };

        let mut created_teams: Vec<OW2Team> = Vec::new();
        let mut extended_teams: Vec<(String, u8, u8, u8, u8, u8, f32)> = Vec::new();
        for team_comp in &teams {
            let team = OW2Team::new(
                team_comp.0.clone(),
                players[&team_comp.1].clone(),
                players[&team_comp.2].clone(),
                players[&team_comp.3].clone(),
                players[&team_comp.4].clone(),
                players[&team_comp.5].clone(),
            );
            let average_sr = team.get_average_sr();
            created_teams.push(team);
            extended_teams.push((
                team_comp.0.clone(),
                team_comp.1,
                team_comp.2,
                team_comp.3,
                team_comp.4,
                team_comp.5,
                average_sr,
            ));
        }

        let number_of_teams = created_teams.len() as f32;

        let mut averages: [f32; 3] = [0.0; 3];
        let mut full_role_average: [f32; 3] = [0.0; 3];
        let mut standard_deviations: [f32; 3] = [0.0; 3];
        let mut average_deviations: [f32; 3] = [0.0; 3];
        let mut team_average_sr: f32 = 0.0;

        let position_vec = vec![Position::Tank, Position::Damage, Position::Support];

        for team in &created_teams {
            let team_sr = team.get_average_sr();
            team_average_sr += team_sr;
        }
        team_average_sr /= number_of_teams;

        let mut sum_of_all_dev_diffs: f32 = 0.0;
        let mut sum_of_all_avg_diffs: f32 = 0.0;

        for position in &position_vec {
            for team in &created_teams {
                averages[*position as usize] += team.get_average_sr_of_role_duo(position.clone());
                standard_deviations[*position as usize] +=
                    team.get_standard_deviation_of_role_duo(position.clone());
            }

            full_role_average[*position as usize] = averages[*position as usize] / number_of_teams;
            average_deviations[*position as usize] =
                standard_deviations[*position as usize] / number_of_teams;

            for team in &created_teams {
                let dev_diff = (team.get_standard_deviation_of_role_duo(position.clone())
                    - average_deviations[*position as usize])
                    .abs();
                sum_of_all_dev_diffs += dev_diff;
                let avg_diff = (team.get_average_sr_of_role_duo(position.clone())
                    - full_role_average[*position as usize])
                    .abs();
                sum_of_all_avg_diffs += avg_diff;
            }
        }

        let rating = (sum_of_all_dev_diffs + sum_of_all_avg_diffs) as i16;

        OW2Matchup {
            kind,
            extended_teams,
            sr_average: team_average_sr,
            full_role_average,
            standard_deviations,
            average_deviations,
            rating,
        }
    }

    pub fn _pretty_print(&self, players: &HashMap<u8, Player>) {
        print!("{}", self.get_pretty_string(players));
    }

    pub fn _get_extended_string(&self, players: &HashMap<u8, Player>) -> String {
        let mut extended_string = self.get_pretty_string(players);

        let position_vec = vec![Position::Tank, Position::Damage, Position::Support];

        let mut created_teams: Vec<OW2Team> = Vec::new();
        for team in &self.extended_teams {
            let team = OW2Team::new(
                team.0.clone(),
                players[&team.1].clone(),
                players[&team.2].clone(),
                players[&team.3].clone(),
                players[&team.4].clone(),
                players[&team.5].clone(),
            );
            created_teams.push(team);
        }

        let mut sum_of_all_avg_diff = 0.0;
        let mut sum_of_all_dev_diff = 0.0;

        for position in &position_vec {
            extended_string.push_str("\n----------------------------------------\n");
            extended_string.push_str(&format!("{:?} Values:\n\n", position));
            extended_string.push_str(&format!(
                "Average SR over all teams: {:.2}\n",
                self.full_role_average[*position as usize]
            ));
            extended_string.push_str(&format!(
                "Average deviation: {:.2}\n\n",
                self.average_deviations[*position as usize]
            ));
            extended_string.push_str("Avg  SR ");
            for team in &created_teams {
                extended_string.push_str(&format!(
                    "{: >25.1}",
                    team.get_average_sr_of_role_duo(position.clone())
                ));
            }
            extended_string.push_str("\n");
            extended_string.push_str("Avg Diff");
            for team in &created_teams {
                let avg_diff = (team.get_average_sr_of_role_duo(position.clone())
                    - self.full_role_average[*position as usize])
                    .abs();
                extended_string.push_str(&format!("{: >25.1}", avg_diff));
                sum_of_all_avg_diff += avg_diff;
            }
            extended_string.push_str("\n\n");
            extended_string.push_str("Dev     ");
            for team in &created_teams {
                extended_string.push_str(&format!(
                    "{: >25.1}",
                    team.get_standard_deviation_of_role_duo(position.clone())
                ));
            }
            extended_string.push_str("\n");
            extended_string.push_str("Dev Diff");
            for team in &created_teams {
                let dev_diff = (team.get_standard_deviation_of_role_duo(position.clone())
                    - self.average_deviations[*position as usize])
                    .abs();
                extended_string.push_str(&format!("{: >25.1}", dev_diff));
                sum_of_all_dev_diff += dev_diff;
            }
            extended_string.push_str("\n");
        }

        extended_string.push_str("\n----------------------------------------\n");
        extended_string.push_str("Global Values\n\n");
        extended_string.push_str(&format!(
            "Sum of all Average differences:   {:.2}\n",
            sum_of_all_avg_diff
        ));
        extended_string.push_str(&format!(
            "Sum of all Deviation differences: {:.2}\n",
            sum_of_all_dev_diff
        ));

        extended_string
    }

    pub fn get_pretty_string(&self, players: &HashMap<u8, Player>) -> String {
        let mut s = String::new();
        s.push_str("-------------------------------------\n");
        s.push_str("OW2Matchup\n");
        s.push_str(&format!("Average SR: {}\n", self.sr_average));
        s.push_str(&format!("Rating: {}\n\n", self.rating));

        let mut team_names = String::new();
        let mut team_sr_averages = String::new();
        let mut tank_line = String::new();
        let mut damage_line_1 = String::new();
        let mut damage_line_2 = String::new();
        let mut support_line_1 = String::new();
        let mut support_line_2 = String::new();

        for team in self.extended_teams.clone() {
            team_names.push_str(&format!("{: >25}", &team.0));
            team_sr_averages.push_str(&format!("{: >25.2}", &team.6));
            tank_line.push_str(
                &format!(
                    "{: >25}",
                    players.get(&team.1).unwrap().print_role(Position::Tank)
                )
                .as_str(),
            );
            damage_line_1.push_str(
                &format!(
                    "{: >25}",
                    players.get(&team.2).unwrap().print_role(Position::Damage)
                )
                .as_str(),
            );
            damage_line_2.push_str(
                &format!(
                    "{: >25}",
                    players.get(&team.3).unwrap().print_role(Position::Damage)
                )
                .as_str(),
            );
            support_line_1.push_str(
                &format!(
                    "{: >25}",
                    players.get(&team.4).unwrap().print_role(Position::Support)
                )
                .as_str(),
            );
            support_line_2.push_str(
                &format!(
                    "{: >25}",
                    players.get(&team.5).unwrap().print_role(Position::Support)
                )
                .as_str(),
            );
        }

        s.push_str(&format!("         {}\n", team_names));
        s.push_str(&format!("         {}\n", team_sr_averages));
        s.push_str(&format!("Tank:    {}\n", tank_line));
        s.push_str(&format!("DPS:     {}\n", damage_line_1));
        s.push_str(&format!("DPS:     {}\n", damage_line_2));
        s.push_str(&format!("Support: {}\n", support_line_1));
        s.push_str(&format!("Support: {}\n", support_line_2));

        s.push_str("-------------------------------------\n");

        s
    }
}
