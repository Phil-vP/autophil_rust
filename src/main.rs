// import Team and Player

use indicatif::ProgressBar;
use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};

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
    let player_map: HashMap<u8, Player> = read_players();

    let number_of_teams: usize = player_map.len() / 6;

    let duos = make_duos(&player_map);

    let matchups = create_matchups(&player_map, &duos, number_of_teams);

    println!("There are {} possible matchups", matchups.len());

    let scrims = create_scrims(&player_map, &matchups, number_of_teams);

    // output the first 10 elements of scrims into a file called "scrims.txt"
    let mut file = File::create("scrims.txt").unwrap();
    for scrim in scrims.iter().take(10) {
        // scrim.pretty_print(&player_map);
        file.write_all(scrim.get_pretty_string(&player_map).as_bytes()).unwrap();
    }
}

// Read all players from players.txt
fn read_players() -> HashMap<u8, Player> {
    // open file players.txt and read all lines that don't start with #
    let mut players: HashMap<u8, Player> = HashMap::new();
    let file = File::open("players.txt").expect("File not found");
    // let mut file = File::open("players_2.txt").expect("File not found");
    let mut i = 1;
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        if !line.starts_with("#") {
            let player = Player::new(line);
            if player.is_ok() {
                players.insert(i, player.unwrap());
                i += 1;
            } else {
                println!("Error with parsing player");
            }
        }
    }
    players
}

fn create_scrims(
    players: &HashMap<u8, Player>,
    matchups: &Vec<(Vec<(u8, u8)>, Vec<(u8, u8)>, Vec<(u8, u8)>)>,
    number_of_teams: usize,
) -> Vec<Matchup> {
    let mut scrims: Vec<Matchup> = Vec::new();

    let mut team_names: Vec<String> = vec!["Naughty Tomatoes".to_string()];

    for _ in team_names.len()..number_of_teams {
        let team_name = format!("Team {}", team_names.len() + 1);
        team_names.push(team_name);
    }

    // team_names.reverse();

    println!("{:?}", team_names);

    let scrim_progress_bar = ProgressBar::new(matchups.len() as u64);
    scrim_progress_bar.reset();

    for possible_matchup in matchups {
        scrim_progress_bar.inc(1);

        let tank_vec = &possible_matchup.0;
        let damage_vec = &possible_matchup.1;
        let support_vec = &possible_matchup.2;
        
        let dps_iter = (0..number_of_teams).permutations(number_of_teams);
        let supp_iter = (0..number_of_teams).permutations(number_of_teams);

        for dps_perm in dps_iter {
            for supp_perm in supp_iter.clone() {
                let mut matchup_teams: Vec<(String, u8, u8, u8, u8, u8, u8)> = Vec::new();
                for i in 0..number_of_teams {
                    matchup_teams.push((
                        team_names[i].clone(),
                        tank_vec.get(i).unwrap().0,
                        tank_vec.get(i).unwrap().1,
                        damage_vec.get(dps_perm[i]).unwrap().0,
                        damage_vec.get(dps_perm[i]).unwrap().1,
                        support_vec.get(supp_perm[i]).unwrap().0,
                        support_vec.get(supp_perm[i]).unwrap().1,
                    ));
                }
                let matchup = Matchup::new(matchup_teams, players);
                scrims.push(matchup);
            }
        }
    }

    scrims.sort_by(|a, b| a.rating.partial_cmp(&b.rating).unwrap());

    scrims
}

fn make_duos(players: &HashMap<u8, Player>) -> HashMap<Position, Vec<(u8, u8)>> {
    let position_vec = vec![Position::Tank, Position::Damage, Position::Support];
    let mut duos: HashMap<Position, Vec<(u8, u8)>> = HashMap::new();

    for position in position_vec {
        let mut role_list: Vec<(u8, &Player)> = Vec::new();
        println!("All players with {:?} role", position);
        for (id, player) in players.iter() {
            if player.plays_position(position) {
                role_list.push((*id, &player));
                println!("{}: {}", id, player.name);
            }
        }
        let tuples: Vec<(u8, u8)> = role_list
            .iter()
            .combinations(2)
            .map(|pair| (pair[0].0, pair[1].0))
            .collect();
        // println!("Position: {:?}, number of pairs: {}", position, tuples.len());
        // for t in &tuples {
        //     println!("({}, {})", players.get(&t.0).unwrap().name, players.get(&t.1).unwrap().name);
        // }
        duos.insert(position, tuples);
    }

    duos
}

fn create_matchups(
    players: &HashMap<u8, Player>,
    player_duos: &HashMap<Position, Vec<(u8, u8)>>,
    number_of_teams: usize,
) -> Vec<(Vec<(u8, u8)>, Vec<(u8, u8)>, Vec<(u8, u8)>)> {
    // {
    let position_vec = vec![Position::Tank, Position::Damage, Position::Support];

    println!("Number of teams: {}", number_of_teams);

    let mut combination_map: HashMap<Position, Vec<Vec<(u8, u8)>>> = HashMap::new();

    for position in &position_vec {
        let combinations_crowded = player_duos[&position]
            .iter()
            .combinations(number_of_teams as usize);
        println!(
            "There are {} possible {:?} combinations",
            combinations_crowded.clone().count(),
            position
        );

        let mut combinations_vec: Vec<Vec<&(u8, u8)>> = combinations_crowded.collect();

        combinations_vec.retain(|c| {
            let mut player_ids: Vec<u8> = Vec::new();
            for player in c {
                player_ids.push(player.0);
                player_ids.push(player.1);
            }
            player_ids.iter().unique().count() == player_ids.len()
        });

        println!("{} of those are unique", combinations_vec.len());

        let mut clean_combinations: Vec<Vec<(u8, u8)>> = Vec::new();

        for combination in &combinations_vec {
            let mut clean_combination: Vec<(u8, u8)> = Vec::new();
            for player_vec in combination {
                // print!("({}, {}); ", players.get(&player_vec.0).unwrap().name, players.get(&player_vec.1).unwrap().name);
                clean_combination.push((player_vec.0, player_vec.1));
            }
            clean_combinations.push(clean_combination);
            // println!("");
        }
        println!();

        combination_map.insert(*position, clean_combinations);
    }

    let number_of_tank_combinations = combination_map[&Position::Tank].len();
    // let number_of_damage_combinations = combination_map[&Position::Damage].len();
    // let number_of_support_combinations = combination_map[&Position::Support].len();

    let tank_progress_bar = ProgressBar::new(number_of_tank_combinations as u64);
    // let damage_progress_bar = ProgressBar::new(number_of_damage_combinations as u64);
    // let support_progress_bar = ProgressBar::new(number_of_support_combinations as u64);

    let mut all_dps_supp_combinations: Vec<(Vec<(u8, u8)>, Vec<(u8, u8)>)> = Vec::new();

    // First calculating all possible Damage & Support pairings
    'damage_outer_loop: for damage_pairs_this_matchup in &combination_map[&Position::Damage] {
        let mut dps_players_seen_this_run: Vec<u8> = Vec::new();
        for dps_duo in damage_pairs_this_matchup {
            dps_players_seen_this_run.push(dps_duo.0);
            dps_players_seen_this_run.push(dps_duo.1);
        }
        // Check if all names are unique
        if dps_players_seen_this_run.iter().unique().count() != dps_players_seen_this_run.len() {
            continue 'damage_outer_loop;
        }
        // All players are unique so far, we can now pair these with Support players
        'support_inner_loop: for support_pairs_this_matchup in &combination_map[&Position::Support]
        {
            let mut dps_and_support_players_seen_this_run = dps_players_seen_this_run.clone();
            for support_duo in support_pairs_this_matchup {
                dps_and_support_players_seen_this_run.push(support_duo.0);
                dps_and_support_players_seen_this_run.push(support_duo.1);
            }
            // Check if all names are unique
            if dps_and_support_players_seen_this_run
                .iter()
                .unique()
                .count()
                != dps_and_support_players_seen_this_run.len()
            {
                continue 'support_inner_loop;
            }

            // Looks like a possible tank / support matchup has been found, it can now be created and appended to the list of all combinations
            let mut dps_supp_combination: (Vec<(u8, u8)>, Vec<(u8, u8)>) = (Vec::new(), Vec::new());
            dps_supp_combination.0 = damage_pairs_this_matchup.clone();
            dps_supp_combination.1 = support_pairs_this_matchup.clone();
            // for duo in &dps_supp_combination {
            //     println!("({}, {})", duo[0].name, duo[1].name);
            // }
            all_dps_supp_combinations.push(dps_supp_combination);
        }
    }

    let mut matchups: Vec<(Vec<(u8, u8)>, Vec<(u8, u8)>, Vec<(u8, u8)>)> = Vec::new();
    //                   | Tank Duos      DPS Duos       Support Duos  |
    //                   +---------------------------------------------+
    //                   |  One Matchup                                |

    tank_progress_bar.reset();
    for tank_pairs_in_this_matchup in &combination_map[&Position::Tank] {
        tank_progress_bar.inc(1);
        let mut tank_names: Vec<u8> = Vec::new();
        for pair in tank_pairs_in_this_matchup {
            tank_names.push(pair.0);
            tank_names.push(pair.1);
        }

        'dps_support_loop: for dps_support_pairs in &all_dps_supp_combinations {
            let mut names_in_this_matchup = tank_names.clone();
            // Add the DPS pairs
            for pair in &dps_support_pairs.0 {
                names_in_this_matchup.push(pair.0);
                names_in_this_matchup.push(pair.1);
            }
            // Add the Support pairs
            for pair in &dps_support_pairs.1 {
                names_in_this_matchup.push(pair.0);
                names_in_this_matchup.push(pair.1);
            }

            // Check if any names are double
            if names_in_this_matchup.iter().unique().count() != names_in_this_matchup.len() {
                continue 'dps_support_loop;
            }

            // No names are double, this is a possibly valid matchup
            let dps_pairs: Vec<(u8, u8)> = dps_support_pairs.0.clone();
            let support_pairs: Vec<(u8, u8)> = dps_support_pairs.1.clone();

            let full_matchup: (Vec<(u8, u8)>, Vec<(u8, u8)>, Vec<(u8, u8)>) =
                (tank_pairs_in_this_matchup.clone(), dps_pairs, support_pairs);

            matchups.push(full_matchup);
        }
    }

    println!("There are {} possible matchups", matchups.len());

    /*
    for matchup in &matchups {
        println!("Tank: ");
        for duo in &matchup.0 {
            print!("({}, {}); ", players.get(&duo.0).unwrap().name, players.get(&duo.1).unwrap().name);
        }
        println!("\nDPS: ");
        for duo in &matchup.1 {
            print!("({}, {}); ", players.get(&duo.0).unwrap().name, players.get(&duo.1).unwrap().name);
        }
        println!("\nSupport: ");
        for duo in &matchup.2 {
            print!("({}, {}); ", players.get(&duo.0).unwrap().name, players.get(&duo.1).unwrap().name);
        }
        println!("\n");
    }
    */

    matchups

    // possible_matchups
}
