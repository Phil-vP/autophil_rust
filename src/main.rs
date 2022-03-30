// import Team and Player

use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use indicatif::{ProgressBar};


mod player;
use player::Player;

mod team;
use team::Team;

mod types;
use types::Position;

mod matchup;
use matchup::Matchup;

use std::collections::HashMap;

fn main() {
    let player_vec: Vec<Player> = read_players();

    let mut matchups: Vec<Vec<&(&Player, &Player)>> = Vec::new();

    // create_matchups(&player_vec, &mut matchups);
    let matchups = create_matchups(&player_vec);

    println!(
        "There are a total of {} possible matchup combinations",
        matchups.len()
    );

    // let teams = create_scrims(&matchups);

    
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
    let file = File::open("players.txt").expect("File not found");
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

fn create_scrims(matchups: &Vec<Vec<&(&Player, &Player)>>) -> Vec<Matchup> {
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

fn create_matchups(players: &Vec<Player>) -> Vec<Vec<&Vec<&&Player>>> {

    let mut all_matchups: Vec<Vec<&Vec<&&Player>>> = Vec::new();

    let number_of_teams: i16 = players.len() as i16 / 6;

    let position_vec = vec![Position::Tank, Position::Damage, Position::Support];

    let mut pair_map: HashMap<Position, Vec<Vec<&&Player>>> = HashMap::new();
    //                                  |   +------------+|
    //                                  |   | Role Duos  ||
    //                                  |   +------------+|
    //                                  +-----------------+
    //                                  | All Duos        |
    //                                  +-----------------+


    for position in position_vec.clone() {
        let role_list: Vec<&Player> = players
            .iter()
            .filter(|player| player.plays_position(position))
            .collect();
        let tuples: Vec<Vec<&&Player>> = role_list
            .iter()
            .combinations(2)
            .map(|pair| vec![pair[0], pair[1]])
            .collect();
        pair_map.insert(position, tuples);
    }

    for position in position_vec.clone() {
        println!(
            "There are {} possible {:?} pairs",
            pair_map[&position].len(),
            position
        );
    }

    println!("Number of teams: {}", number_of_teams);

    let mut combination_map: HashMap<Position, Vec<Vec<&Vec<&&Player>>>> = HashMap::new();
    //                                         |   |   +------------+||
    //                                         |   |   | Role Duos  |||
    //                                         |   |   +------------+||
    //                                         |   +-----------------+|
    //                                         |   | Duo Combination ||
    //                                         |   +-----------------+|
    //                                         +----------------------+
    //                                         |  All combinations    |
    //                                         +----------------------+

    for position in position_vec.clone() {
        let combinations_crowded = pair_map[&position]
            .iter()
            .combinations(number_of_teams as usize);
        println!("There are {} {:?} combinations", combinations_crowded.clone().count(), position);
        
        let combinations_clean = combinations_crowded.filter(|c| {
            let mut player_names: Vec<String> = Vec::new();
            for player in c {
                player_names.push(player[0].name.clone());
                player_names.push(player[1].name.clone());
            }
            player_names.iter().unique().count() == player_names.len()
        });

        let clean_combinations: Vec<Vec<&Vec<&&Player>>> = combinations_clean.collect();

        combination_map.insert(position, clean_combinations);
        println!("{} of those are unique\n", combination_map[&position].len());
    }


    let number_of_tank_combinations = combination_map[&Position::Tank].len();

    let tank_progress_bar = ProgressBar::new(number_of_tank_combinations as u64);

    let mut all_dps_supp_combinations: Vec<Vec<&Vec<&&Player>>> = Vec::new();

    // First calculating all possible Damage & Support pairings
    'damage_outer_loop: for damage_pairs_this_matchup in combination_map[&Position::Damage].clone() {
        let mut dps_players_seen_this_run: Vec<String> = Vec::new();
        for dps_duo in &damage_pairs_this_matchup {
            dps_players_seen_this_run.push(dps_duo[0].name.clone());
            dps_players_seen_this_run.push(dps_duo[1].name.clone());
        }
        println!("DPS Players this run: {:?}", dps_players_seen_this_run);
        // Check if all names are unique
        if dps_players_seen_this_run.iter().unique().count() != dps_players_seen_this_run.len() {
            continue 'damage_outer_loop;
        }
        // All players are unique so far, we can now pair these with Support players
        'support_inner_loop: for support_pairs_this_matchup in combination_map[&Position::Support].clone() {
            let mut dps_and_support_players_seen_this_run = dps_players_seen_this_run.clone();
            for support_duo in &support_pairs_this_matchup {
                dps_and_support_players_seen_this_run.push(support_duo[0].name.clone());
                dps_and_support_players_seen_this_run.push(support_duo[1].name.clone());
            }
            println!("    DPS and Support Players this run: {:?}", dps_and_support_players_seen_this_run);
            // Check if all names are unique
            if dps_and_support_players_seen_this_run.iter().unique().count() != dps_and_support_players_seen_this_run.len() {
                println!("    Skipping");
                continue 'support_inner_loop;
            }

            // Looks like a possible tank / support matchup has been found, it can now be created and appended to the list of all combinations
            let mut dps_supp_combination: Vec<&Vec<&&Player>> = Vec::new();
            dps_supp_combination.extend(damage_pairs_this_matchup.clone());
            dps_supp_combination.extend(support_pairs_this_matchup.clone());
            println!("    Success");
            // for duo in &dps_supp_combination {
            //     println!("({}, {})", duo.0.name, duo.1.name);
            // }
            all_dps_supp_combinations.push(dps_supp_combination);
        }
    }

    println!("There are {} possible DPS / Support pairs", all_dps_supp_combinations.len());

    tank_progress_bar.reset();
    for tank_pairs_in_this_matchup in &combination_map[&Position::Tank] {
        tank_progress_bar.inc(1);
        let mut tank_names:Vec<String> = Vec::new();
        for pair in tank_pairs_in_this_matchup {
            tank_names.push(pair[0].name.clone());
            tank_names.push(pair[1].name.clone());
        }

        'dps_support_loop: for dps_support_pairs in &all_dps_supp_combinations {
            let mut names_in_this_matchup = tank_names.clone();
            for pair in dps_support_pairs {
                names_in_this_matchup.push(pair[0].name.clone());
                names_in_this_matchup.push(pair[1].name.clone());
            }

            // Check if any names are double
            if names_in_this_matchup.iter().unique().count() != names_in_this_matchup.len() {
                continue 'dps_support_loop;
            }

            println!("Found matchup: {:?}", names_in_this_matchup);

            let mut full_matchup: Vec<&Vec<&&Player>> = tank_pairs_in_this_matchup.clone();
            full_matchup.extend(dps_support_pairs);
            println!("Matchup found with length {}:" , full_matchup.len());
            for duo in &full_matchup {
                println!("({}, {})", duo[0].name, duo[1].name);
            }

            // No names are double, this is a possibly valid matchup
            // matchups.push(full_matchup);

            all_matchups.push(full_matchup);
        }
    }

    all_matchups

    /*
    tank_progress_bar.reset();
    'tank_loop: for tank_pairs_in_this_matchup in combination_map[&Position::Tank].clone() {
        // damage_progress_bar.reset();
        tank_progress_bar.inc(1);
        let mut all_players_this_run_only_tanks: Vec<String> = Vec::new();

        for tank_pair in tank_pairs_in_this_matchup.clone() {
            // println!("Checking tank pair: ({}, {})", tank_pair.0.name, tank_pair.1.name);
            all_players_this_run_only_tanks.push(tank_pair.0.clone().name);
            all_players_this_run_only_tanks.push(tank_pair.1.clone().name);
        }
        // If not all names are unique, continue with tank_loop
        if all_players_this_run_only_tanks.iter().unique().count() != all_players_this_run_only_tanks.len() {
            continue 'tank_loop;
        }

        // println!("all_players_this_run_only_tanks: {:?}", all_players_this_run_only_tanks);

        'damage_loop: for damage_pairs_in_this_matchup in combination_map[&Position::Damage].clone() {
            // support_progress_bar.reset();
            // damage_progress_bar.inc(1);
            let mut all_players_this_run_tanks_and_dps = all_players_this_run_only_tanks.clone();
            for damage_pair in damage_pairs_in_this_matchup.clone() {
                // println!("Checking damage pair: ({},{})", damage_pair.0.name, damage_pair.1.name);
                if all_players_this_run_tanks_and_dps.contains(&damage_pair.0.name) {
                    // println!("all_players_this_run_tanks_and_dps: {:?}", all_players_this_run_tanks_and_dps);
                    // println!("{} is already in the list", damage_pair.0.name);
                    continue 'damage_loop;
                }
                if all_players_this_run_tanks_and_dps.contains(&damage_pair.1.name) {
                    // println!("all_players_this_run_tanks_and_dps: {:?}", all_players_this_run_tanks_and_dps);
                    // println!("{} is already in the list", damage_pair.1.name);
                    continue 'damage_loop;
                }
                all_players_this_run_tanks_and_dps.push(damage_pair.0.clone().name);
                all_players_this_run_tanks_and_dps.push(damage_pair.1.clone().name);
            }

            // If not all names are unique, continue with damage_loop
            if all_players_this_run_tanks_and_dps.iter().unique().count() != all_players_this_run_tanks_and_dps.len() {
                continue 'damage_loop;
            }
            // println!("all_players_this_run_tanks_and_dps: {:?}", all_players_this_run_tanks_and_dps);

            'support_loop: for support_pairs_in_this_matchup in combination_map[&Position::Support].clone() {
                // support_progress_bar.inc(1);
                let mut all_players_this_run_tanks_and_dps_and_supports =
                    all_players_this_run_tanks_and_dps.clone();
                for support_pair in support_pairs_in_this_matchup.clone() {
                    // println!("Checking support pair: ({},{})", support_pair.0.name, support_pair.1.name);
                    all_players_this_run_tanks_and_dps_and_supports
                        .push(support_pair.0.clone().name);
                    all_players_this_run_tanks_and_dps_and_supports
                        .push(support_pair.1.clone().name);
                }
                // If not all names are unique, continue with support_loop
                if all_players_this_run_tanks_and_dps_and_supports
                    .iter()
                    .unique()
                    .count()
                    != all_players_this_run_tanks_and_dps_and_supports.len()
                {
                    continue 'support_loop;
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
                matchups.push(matchup_vec);
            }
        }
    }
    */

    // possible_matchups
}
