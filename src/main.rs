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

    let number_of_teams: usize = player_vec.len() / 6;
    
    let duos = make_duos(&player_vec);

    create_matchups(&duos, number_of_teams);

    /*

    println!(
        "There are a total of {} possible matchup combinations",
        matchups.len()
    );

    let teams = create_scrims(&matchups);
*/
    
    /*
    let mut i = 0;

    for matchup in matchups {
        i += 1;
        for tuple in matchup {
            println!("{}", tuple[0].name);
            println!("{}", tuple[1].name);
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

fn create_scrims(matchups: &Vec<&Vec<&(&Player, &Player)>>) -> Vec<Matchup> {
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

fn make_duos(players: &Vec<Player>) -> HashMap<Position, Vec<Vec<&Player>>> {
    let position_vec = vec![Position::Tank, Position::Damage, Position::Support];
    let mut duos: HashMap<Position, Vec<Vec<&Player>>> = HashMap::new();

    for position in position_vec {
        let role_list: Vec<&Player> = players
        .iter()
        .filter(|player| player.plays_position(position))
        .collect();
        let tuples: Vec<Vec<&Player>> = role_list
            .iter()
            .combinations(2)
            .map(|pair| vec![*pair[0], *pair[1]])
            .collect();
        println!("Position: {:?}, number of pairs: {}", position, tuples.len());
        for t in &tuples {
            println!("({}, {})", t.get(0).unwrap().name, t.get(1).unwrap().name);
        }
        duos.insert(position, tuples);
    }

    duos
}


// fn create_matchups<'a>(player_duos: &'a HashMap<Position, Vec<&Player>>, number_of_teams: usize, matchups: &'a mut Vec<&Vec<&(&Player, &Player)>>) -> () {
fn create_matchups<'a>(player_duos: &'a HashMap<Position, Vec<Vec<&Player>>>, number_of_teams: usize) -> () { // {
    let position_vec = vec![Position::Tank, Position::Damage, Position::Support];

    println!("Number of teams: {}", number_of_teams);

    let mut combination_map: HashMap<Position, Vec<Vec<&Vec<&Player>>>> = HashMap::new();

    for position in &position_vec {
        let combinations_crowded = player_duos[&position]
            .iter()
            .combinations(number_of_teams as usize);
        println!("There are {} {:?} combinations", combinations_crowded.clone().count(), position);

        let mut combinations_vec: Vec<Vec<&Vec<&Player>>> = combinations_crowded.collect();
        
        combinations_vec.retain(|c| {
            let mut player_names: Vec<String> = Vec::new();
            for player in c {
                player_names.push(player[0].name.clone());
                player_names.push(player[1].name.clone());
            }
            player_names.iter().unique().count() == player_names.len()
        });

        println!("{} of those are unique:", combinations_vec.len());
        
        for combination in &combinations_vec {
            for player_vec in combination {
                print!("({}, {}) ; ", player_vec[0].name, player_vec[1].name);
            }
            println!("");
        }
        println!();

        combination_map.insert(*position, combinations_vec);
    }


    let number_of_tank_combinations = combination_map[&Position::Tank].len();
    // let number_of_damage_combinations = combination_map[&Position::Damage].len();
    // let number_of_support_combinations = combination_map[&Position::Support].len();

    let tank_progress_bar = ProgressBar::new(number_of_tank_combinations as u64);
    // let damage_progress_bar = ProgressBar::new(number_of_damage_combinations as u64);
    // let support_progress_bar = ProgressBar::new(number_of_support_combinations as u64);

    let mut all_dps_supp_combinations: Vec<Vec<&Vec<&Player>>> = Vec::new();

    // First calculating all possible Damage & Support pairings
    'damage_outer_loop: for damage_pairs_this_matchup in &combination_map[&Position::Damage] {
        // let mut dps_supp_combination: Vec<&(&Player, &Player)> = Vec::new();
        let mut dps_players_seen_this_run: Vec<String> = Vec::new();
        for dps_duo in damage_pairs_this_matchup {
            dps_players_seen_this_run.push(dps_duo[0].name.clone());
            dps_players_seen_this_run.push(dps_duo[1].name.clone());
        }
        // Check if all names are unique
        if dps_players_seen_this_run.iter().unique().count() != dps_players_seen_this_run.len() {
            continue 'damage_outer_loop;
        }
        // All players are unique so far, we can now pair these with Support players
        'support_inner_loop: for support_pairs_this_matchup in &combination_map[&Position::Support] {
            let mut dps_and_support_players_seen_this_run = dps_players_seen_this_run.clone();
            for support_duo in support_pairs_this_matchup {
                dps_and_support_players_seen_this_run.push(support_duo[0].name.clone());
                dps_and_support_players_seen_this_run.push(support_duo[1].name.clone());
            }
            // Check if all names are unique
            if dps_and_support_players_seen_this_run.iter().unique().count() != dps_and_support_players_seen_this_run.len() {
                continue 'support_inner_loop;
            }

            // Looks like a possible tank / support matchup has been found, it can now be created and appended to the list of all combinations
            let mut dps_supp_combination: Vec<&Vec<&Player>> = Vec::new();
            dps_supp_combination.extend(damage_pairs_this_matchup);
            dps_supp_combination.extend(support_pairs_this_matchup);
            // for duo in &dps_supp_combination {
            //     println!("({}, {})", duo[0].name, duo[1].name);
            // }
            all_dps_supp_combinations.push(dps_supp_combination);
        }
    }

    println!("There are {} possible DPS / Support pairs:", all_dps_supp_combinations.len());
    for dps_supp_combination in &all_dps_supp_combinations {
        for duo in dps_supp_combination {
            print!("({}, {}) & ", duo[0].name, duo[1].name);
        }
        println!();
    }

    let mut matchups: Vec<(&Vec<&Vec<&Player>>, &Vec<&Vec<&Player>>, &Vec<&Vec<&Player>>)>;
    //                    |Tank Duos            DPS Duos             Support Duos       |
    //                    +-------------------------------------------------------------+
    //                    |  One Matchup                                                |

    
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

            let dps_pairs: &Vec<&Vec<&Player>> = dps_support_pairs[0..dps_support_pairs.len()/2].to_vec();
            let support_pairs: &Vec<&Vec<&Player>> = dps_support_pairs[dps_support_pairs.len()/2..].to_vec();

            let mut full_matchup: (&Vec<&Vec<&Player>>, &Vec<&Vec<&Player>>, &Vec<&Vec<&Player>>) = (tank_pairs_in_this_matchup, dps_pairs, support_pairs);

            // No names are double, this is a possibly valid matchup
            matchups.push(full_matchup);

        }
    }
    
    

    // possible_matchups
}
