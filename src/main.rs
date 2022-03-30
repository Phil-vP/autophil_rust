// import Team and Player

use anyhow::Result;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod player;
use player::Player;

mod team;
use team::Team;

mod types;
use types::Position;

mod matchup;
use matchup::Matchup;

fn main() {
    let player_vec: Vec<Player> = read_players();

    let matchups = create_matchups(player_vec);

    println!(
        "There is a total of {} possible matchup combinations",
        matchups.len()
    );

    let teams = create_scrims(matchups);



    /*
    let mut i = 0;

    for matchup in matchups {
        i += 1;
        for tuple in matchup {
            println!("{}", tuple.0.name);
            println!("{}", tuple.1.name);
        }
        println!("");

        if i > 5 {
            break;
        }
    }
    */
}

// Read all players from players.txt
fn read_players() -> Vec<Player> {
    // open file players.txt and read all lines that don't start with #
    let mut players: Vec<Player> = Vec::new();
    let mut file = File::open("players.txt").expect("File not found");
    // let mut file = File::open("players_2.txt").expect("File not found");
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        if !line.starts_with("#") {
            let player = Player::new(line);
            if player.is_ok() {
                players.push(player.unwrap());
            } else {
                println!("Error with parsing player");
            }
        }
    }
    players
}

fn create_scrims(matchups: Vec<Vec<(Player, Player)>>) -> Vec<Matchup> {
    let mut scrims: Vec<Matchup> = Vec::new();

    let mut team_names: Vec<String> = vec!["Naughty Tomatoes".to_string()];

    let number_of_teams = (matchups[0].len() / 3) as i16;

    for _ in (team_names.len() as i16)..number_of_teams {
        let team_name = format!("Team {}", team_names.len() + 1);
        team_names.push(team_name);
    }

    team_names.reverse();

    println!("{:?}", team_names);

    scrims
}

fn create_matchups(players: Vec<Player>) -> Vec<Vec<(Player, Player)>> {
    
    let number_of_teams: i16 = players.len() as i16 / 6;

    let mut possible_matchups: Vec<Vec<(Player, Player)>> = Vec::new();

    let tank_list = players
        .iter()
        .filter(|p| p.plays_position(Position::Tank))
        .collect::<Vec<&Player>>();

    let damage_list = players
        .iter()
        .filter(|p| p.plays_position(Position::Damage))
        .collect::<Vec<&Player>>();

    let support_list = players
        .iter()
        .filter(|p| p.plays_position(Position::Support))
        .collect::<Vec<&Player>>();

    let all_tank_pairs = tank_list.iter().combinations(2);
    let mut all_tank_tuples: Vec<(&Player, &Player)> = Vec::new();
    println!("Tank pairs: ");
    for tank_pair in all_tank_pairs.clone() {
        let tuple = (tank_pair[0].clone(), tank_pair[1].clone());
        all_tank_tuples.push(tuple);
        println!("({}, {})", tuple.0.name, tuple.1.name);
    }

    let all_damage_pairs = damage_list.iter().combinations(2);
    let mut all_damage_tuples: Vec<(&Player, &Player)> = Vec::new();
    println!("Damage pairs: ");
    for damage_pair in all_damage_pairs.clone() {
        let tuple = (damage_pair[0].clone(), damage_pair[1].clone());
        all_damage_tuples.push(tuple);
        println!("({}, {})", tuple.0.name, tuple.1.name);
    }

    let all_support_pairs = support_list.iter().combinations(2);
    let mut all_support_tuples: Vec<(&Player, &Player)> = Vec::new();
    println!("Support pairs: ");
    for support_pair in all_support_pairs.clone() {
        let tuple = (support_pair[0].clone(), support_pair[1].clone());
        all_support_tuples.push(tuple);
        println!("({}, {})", tuple.0.name, tuple.1.name);
    }

    println!("There are {} tank pairs", all_tank_tuples.clone().len());
    println!("There are {} damage pairs", all_damage_tuples.clone().len());
    println!(
        "There are {} support pairs",
        all_support_tuples.clone().len()
    );

    println!("Number of teams: {}", number_of_teams);

    let tank_combinations = all_tank_tuples
        .iter()
        .combinations(number_of_teams as usize);
    let damage_combinations = all_damage_tuples
        .iter()
        .combinations(number_of_teams as usize);
    let support_combinations = all_support_tuples
        .iter()
        .combinations(number_of_teams as usize);

    'tank_loop: for tank_pairs_in_this_matchup in tank_combinations {
        let mut all_players_this_run_only_tanks: Vec<String> = Vec::new();

        for tank_pair in tank_pairs_in_this_matchup.clone() {
            if all_players_this_run_only_tanks.contains(&tank_pair.0.name) {
                continue 'tank_loop;
            }
            all_players_this_run_only_tanks.push(tank_pair.0.clone().name);
            if all_players_this_run_only_tanks.contains(&tank_pair.1.name) {
                continue 'tank_loop;
            }
            all_players_this_run_only_tanks.push(tank_pair.1.clone().name);
        }

        // println!("all_players_this_run_only_tanks: {:?}", all_players_this_run_only_tanks);

        'damage_loop: for damage_pairs_in_this_matchup in damage_combinations.clone() {
            let mut all_players_this_run_tanks_and_dps = all_players_this_run_only_tanks.clone();
            for damage_pair in damage_pairs_in_this_matchup.clone() {
                if all_players_this_run_tanks_and_dps.contains(&damage_pair.0.name) {
                    // println!("all_players_this_run_tanks_and_dps: {:?}", all_players_this_run_tanks_and_dps);
                    // println!("{} is already in the list", damage_pair.0.name);
                    continue 'damage_loop;
                }
                all_players_this_run_tanks_and_dps.push(damage_pair.0.clone().name);
                if all_players_this_run_tanks_and_dps.contains(&damage_pair.1.name) {
                    // println!("all_players_this_run_tanks_and_dps: {:?}", all_players_this_run_tanks_and_dps);
                    // println!("{} is already in the list", damage_pair.1.name);
                    continue 'damage_loop;
                }
                all_players_this_run_tanks_and_dps.push(damage_pair.1.clone().name);
            }
            // println!("all_players_this_run_tanks_and_dps: {:?}", all_players_this_run_tanks_and_dps);

            'support_loop: for support_pairs_in_this_matchup in support_combinations.clone() {
                let mut all_players_this_run_tanks_and_dps_and_supports =
                    all_players_this_run_tanks_and_dps.clone();
                for support_pair in support_pairs_in_this_matchup.clone() {
                    if all_players_this_run_tanks_and_dps_and_supports
                        .contains(&support_pair.0.name)
                    {
                        continue 'support_loop;
                    }
                    all_players_this_run_tanks_and_dps_and_supports
                        .push(support_pair.0.clone().name);
                    if all_players_this_run_tanks_and_dps_and_supports
                        .contains(&support_pair.1.name)
                    {
                        continue 'support_loop;
                    }
                    all_players_this_run_tanks_and_dps_and_supports
                        .push(support_pair.1.clone().name);
                }
                // No double players available, this is a possible combination of pairs
                let mut matchup_vec: Vec<(Player, Player)> = Vec::new();
                for tank_pair in tank_pairs_in_this_matchup.clone() {
                    matchup_vec.push((tank_pair.0.clone(), tank_pair.1.clone()));
                }
                for damage_pair in damage_pairs_in_this_matchup.clone() {
                    matchup_vec.push((damage_pair.0.clone(), damage_pair.1.clone()));
                }
                for support_pair in support_pairs_in_this_matchup.clone() {
                    matchup_vec.push((support_pair.0.clone(), support_pair.1.clone()));
                }
                possible_matchups.push(matchup_vec);
            }
        }
    }

    possible_matchups
}
