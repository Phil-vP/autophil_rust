// import Team and Player

use indicatif::ProgressBar;
use itertools::Itertools;
use std::cmp;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;

mod player;
use player::Player;

mod team;
use team::Team;

mod team_ow2;
use team_ow2::OW2Team;

mod types;
use types::Position;

mod matchup;
use matchup::Matchup;

mod matchup_ow2;
use matchup_ow2::OW2Matchup;

use std::collections::HashMap;

fn main() {
    let player_map: HashMap<u8, Player> = read_players();

    let number_of_teams: usize = player_map.len() / 6;

    let ow_2 = false;

    let number_of_printed_scrims = 10;

    let number_of_threads = 16;

    let duos = make_duos(&player_map, ow_2);

    let mut team_names: Vec<String> = vec!["Naughty Tomatoes".to_string()];

    for _ in team_names.len()..number_of_teams {
        let team_name = format!("Team {}", team_names.len() + 1);
        team_names.push(team_name);
    }

    if ow_2 {
        let matchups = create_ow2_matchups(&duos, number_of_teams, number_of_threads);
        println!("There are {} possible matchups", matchups.len());
        let scrims = create_ow2_scrims(
            &player_map,
            matchups,
            number_of_teams,
            team_names,
            number_of_threads,
        );

        let mut file = File::create("scrims.txt").unwrap();
        let mut i: u8 = 0;
        for scrim in scrims.iter().take(number_of_printed_scrims) {
            file.write(format!("SCRIM {}:\n", (65 + i) as char).as_bytes())
                .unwrap();
            file.write_all(scrim.get_pretty_string(&player_map).as_bytes())
                .unwrap();
            i += 1;
        }
    } else {
        let matchups = create_matchups(&duos, number_of_teams, number_of_threads);
        println!("There are {} possible matchups", matchups.len());
        let scrims = create_scrims(
            &player_map,
            matchups,
            number_of_teams,
            team_names,
            number_of_threads,
        );

        let mut file = File::create("scrims.txt").unwrap();
        let mut i: u8 = 0;
        for scrim in scrims.iter().take(number_of_printed_scrims) {
            file.write(format!("SCRIM {}:\n", (65 + i) as char).as_bytes())
                .unwrap();
            file.write_all(scrim.get_pretty_string(&player_map).as_bytes())
                .unwrap();
            i += 1;
        }
    };
}

// Read all players from players.txt
fn read_players() -> HashMap<u8, Player> {
    // open file players.txt and read all lines that don't start with #
    let mut players: HashMap<u8, Player> = HashMap::new();
    // let file = File::open("players_balanced.txt").expect("File not found");
    let file = File::open("players.txt").expect("File not found");
    // let file = File::open("players_2.txt").expect("File not found");
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
    players_raw: &HashMap<u8, Player>,
    matchups: Vec<(Vec<(u8, u8)>, Vec<(u8, u8)>, Vec<(u8, u8)>)>,
    number_of_teams: usize,
    team_names_raw: Vec<String>,
    number_of_threads: usize,
) -> Vec<Matchup> {
    let scrims: Vec<Vec<Matchup>> = Vec::new();

    println!("{:?}", team_names_raw);

    let number_of_matchups = matchups.len();

    let scrim_progress_bar = ProgressBar::new(number_of_matchups as u64);
    scrim_progress_bar.reset();

    let chunk_size = number_of_matchups / number_of_threads;

    let matchup_chunks = matchups.into_iter().chunks(chunk_size);

    let mut handles = vec![];

    let mutex = Mutex::new((scrims, scrim_progress_bar));
    let arc = Arc::new(mutex);

    for matchup_chunk in matchup_chunks.into_iter() {
        let matchup_chunk: Vec<(Vec<(u8, u8)>, Vec<(u8, u8)>, Vec<(u8, u8)>)> =
            matchup_chunk.collect();

        let cloned_arc = Arc::clone(&arc);
        let players = players_raw.clone();
        let team_names = team_names_raw.clone();

        let mut best_rating: i16 = i16::MAX;

        let handle = thread::spawn(move || {
            let mut all_scrims: Vec<Matchup> = Vec::new();
            let mut counter = 0;
            for possible_matchup in matchup_chunk {
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
                        let matchup = Matchup::new(matchup_teams, &players);
                        let rating = matchup.rating;
                        if rating < (best_rating as f32 * 1.1) as i16 {
                            all_scrims.push(matchup);
                            best_rating = cmp::min(rating, best_rating);
                        }
                    }
                }
                counter += 1;
                if (counter % 1000) == 0 {
                    cloned_arc.lock().unwrap().1.inc(1000);
                }
            }
            // println!("Adding {} scrims to the shared vector", all_scrims.len());
            let all_scrims_vector = &mut cloned_arc.lock().unwrap().0;
            all_scrims_vector.push(all_scrims);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let scrim_vector = arc.lock().unwrap().0.clone();
    let mut scrims: Vec<Matchup> = Vec::new();
    for mut scrim_chunk in scrim_vector {
        scrims.append(&mut scrim_chunk);
    }

    arc.lock().unwrap().1.finish();

    println!("\nTotal number of scrims: {}", scrims.len());
    scrims.sort_by(|a, b| a.rating.partial_cmp(&b.rating).unwrap());

    scrims
}

fn make_duos(players: &HashMap<u8, Player>, ow_2: bool) -> HashMap<Position, Vec<(u8, u8)>> {
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

        let tuples: Vec<(u8, u8)> = if ow_2 && position == Position::Tank {
            // collect the role list into a list of tuples with (id, 0)
            role_list.iter().map(|(id, _)| (*id, 0)).collect()
        } else {
            role_list
                .iter()
                .combinations(2)
                .map(|pair| (pair[0].0, pair[1].0))
                .collect()
        };
        // println!("Position: {:?}, number of pairs: {}", position, tuples.len());
        // for t in &tuples {
        //     println!("({}, {})", players.get(&t.0).unwrap().name, players.get(&t.1).unwrap().name);
        // }
        duos.insert(position, tuples);
    }

    duos
}

fn create_matchups(
    player_duos: &HashMap<Position, Vec<(u8, u8)>>,
    number_of_teams: usize,
    number_of_threads: usize,
) -> Vec<(Vec<(u8, u8)>, Vec<(u8, u8)>, Vec<(u8, u8)>)> {
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
                clean_combination.push((player_vec.0, player_vec.1));
            }
            clean_combinations.push(clean_combination);
        }
        println!();

        combination_map.insert(*position, clean_combinations);
    }

    let number_of_tank_combinations = combination_map[&Position::Tank].len();

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
            all_dps_supp_combinations.push(dps_supp_combination);
        }
    }

    println!("Starting to combine the tank matchups");

    let matchup_vec: Vec<Vec<(Vec<(u8, u8)>, Vec<(u8, u8)>, Vec<(u8, u8)>)>> = Vec::new();
    //                          | Tank Duos      DPS Duos       Support Duos  |
    //                          +---------------------------------------------+
    //                          |  One Matchup                                |

    let tank_progress_bar = ProgressBar::new(number_of_tank_combinations as u64);
    tank_progress_bar.reset();

    let chunk_size = number_of_tank_combinations / number_of_threads;

    let tank_chunks = &combination_map[&Position::Tank]
        .clone()
        .into_iter()
        .chunks(chunk_size);

    let mut handles = vec![];

    let mutex = Mutex::new((matchup_vec, tank_progress_bar));
    let arc = Arc::new(mutex);

    for tank_chunk in tank_chunks.into_iter() {
        let tank_chunk: Vec<Vec<(u8, u8)>> = tank_chunk.collect();

        let dps_supp_combinations = all_dps_supp_combinations.clone();

        // println!("length of matchup chunk: {}", matchup_chunk.len());

        let cloned_arc = Arc::clone(&arc);

        let handle = thread::spawn(move || {
            let mut matchups_this_thread: Vec<(Vec<(u8, u8)>, Vec<(u8, u8)>, Vec<(u8, u8)>)> =
                Vec::new();
            let mut counter = 0;

            for tank_pairs_in_this_matchup in tank_chunk {
                let mut tank_names: Vec<u8> = Vec::new();
                for pair in &tank_pairs_in_this_matchup {
                    tank_names.push(pair.0);
                    tank_names.push(pair.1);
                }

                'dps_support_loop: for dps_support_pairs in &dps_supp_combinations {
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
                    if names_in_this_matchup.iter().unique().count() != names_in_this_matchup.len()
                    {
                        continue 'dps_support_loop;
                    }

                    // No names are double, this is a possibly valid matchup
                    let dps_pairs: Vec<(u8, u8)> = dps_support_pairs.0.clone();
                    let support_pairs: Vec<(u8, u8)> = dps_support_pairs.1.clone();

                    let full_matchup: (Vec<(u8, u8)>, Vec<(u8, u8)>, Vec<(u8, u8)>) =
                        (tank_pairs_in_this_matchup.clone(), dps_pairs, support_pairs);

                    matchups_this_thread.push(full_matchup);
                }
                counter += 1;
                if (counter % 10) == 0 {
                    cloned_arc.lock().unwrap().1.inc(10);
                }
            }
            cloned_arc.lock().unwrap().0.push(matchups_this_thread);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let matchup_vector = arc.lock().unwrap().0.clone();
    let mut matchups: Vec<(Vec<(u8, u8)>, Vec<(u8, u8)>, Vec<(u8, u8)>)> = Vec::new();
    for mut matchup_chunk in matchup_vector {
        matchups.append(&mut matchup_chunk);
    }

    // println!("There are {} possible matchups", matchups.len());

    matchups
}

fn create_ow2_scrims(
    players_raw: &HashMap<u8, Player>,
    matchups: Vec<(Vec<u8>, Vec<(u8, u8)>, Vec<(u8, u8)>)>,
    number_of_teams: usize,
    team_names_raw: Vec<String>,
    number_of_threads: usize,
) -> Vec<OW2Matchup> {
    let scrims: Vec<Vec<OW2Matchup>> = Vec::new();

    println!("{:?}", team_names_raw);

    let number_of_matchups = matchups.len();

    let scrim_progress_bar = ProgressBar::new(number_of_matchups as u64);
    scrim_progress_bar.reset();

    let chunk_size = number_of_matchups / number_of_threads;

    let matchup_chunks = matchups.into_iter().chunks(chunk_size);

    let mut handles = vec![];

    let mutex = Mutex::new((scrims, scrim_progress_bar));
    let arc = Arc::new(mutex);

    for matchup_chunk in matchup_chunks.into_iter() {
        let matchup_chunk: Vec<(Vec<u8>, Vec<(u8, u8)>, Vec<(u8, u8)>)> = matchup_chunk.collect();

        let cloned_arc = Arc::clone(&arc);
        let players = players_raw.clone();
        let team_names = team_names_raw.clone();

        let handle = thread::spawn(move || {
            let mut all_scrims: Vec<OW2Matchup> = Vec::new();
            let mut counter = 0;

            let mut best_rating: i16 = i16::MAX;

            for possible_matchup in matchup_chunk {
                let tank_vec = &possible_matchup.0;
                let damage_vec = &possible_matchup.1;
                let support_vec = &possible_matchup.2;

                let dps_iter = (0..number_of_teams).permutations(number_of_teams);
                let supp_iter = (0..number_of_teams).permutations(number_of_teams);

                for dps_perm in dps_iter {
                    for supp_perm in supp_iter.clone() {
                        let mut matchup_teams: Vec<(String, u8, u8, u8, u8, u8)> = Vec::new();
                        for i in 0..number_of_teams {
                            matchup_teams.push((
                                team_names[i].clone(),
                                *tank_vec.get(i).unwrap(),
                                damage_vec.get(dps_perm[i]).unwrap().0,
                                damage_vec.get(dps_perm[i]).unwrap().1,
                                support_vec.get(supp_perm[i]).unwrap().0,
                                support_vec.get(supp_perm[i]).unwrap().1,
                            ));
                        }
                        let matchup = OW2Matchup::new(matchup_teams, &players);
                        let rating = matchup.rating;
                        if rating < (best_rating as f32 * 1.1) as i16 {
                            all_scrims.push(matchup);
                            best_rating = cmp::min(rating, best_rating);
                        }
                    }
                }
                counter += 1;
                if (counter % 1000) == 0 {
                    cloned_arc.lock().unwrap().1.inc(1000);
                }
            }
            let all_scrims_vector = &mut cloned_arc.lock().unwrap().0;
            all_scrims_vector.push(all_scrims);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let scrim_vector = arc.lock().unwrap().0.clone();
    let mut scrims: Vec<OW2Matchup> = Vec::new();
    for mut scrim_chunk in scrim_vector {
        scrims.append(&mut scrim_chunk);
    }

    arc.lock().unwrap().1.finish();

    println!("\nTotal number of scrims: {}", scrims.len());
    scrims.sort_by(|a, b| a.rating.partial_cmp(&b.rating).unwrap());

    scrims
}

fn create_ow2_matchups(
    player_duos: &HashMap<Position, Vec<(u8, u8)>>,
    number_of_teams: usize,
    number_of_threads: usize,
) -> Vec<(Vec<u8>, Vec<(u8, u8)>, Vec<(u8, u8)>)> {
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
            position == &Position::Tank || (player_ids.iter().unique().count() == player_ids.len())
        });

        println!("{} of those are unique", combinations_vec.len());

        let mut clean_combinations: Vec<Vec<(u8, u8)>> = Vec::new();

        for combination in &combinations_vec {
            let mut clean_combination: Vec<(u8, u8)> = Vec::new();
            for player_vec in combination {
                clean_combination.push((player_vec.0, player_vec.1));
            }
            clean_combinations.push(clean_combination);
        }
        println!();

        combination_map.insert(*position, clean_combinations);
    }

    let number_of_tank_combinations = combination_map[&Position::Tank].len();

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
            all_dps_supp_combinations.push(dps_supp_combination);
        }
    }

    let matchup_vec: Vec<Vec<(Vec<u8>, Vec<(u8, u8)>, Vec<(u8, u8)>)>> = Vec::new();
    //                      | Tanks    DPS Duos       Support Duos  |
    //                      +---------------------------------------+
    //                      |  One Matchup                          |

    let tank_progress_bar = ProgressBar::new(number_of_tank_combinations as u64);
    tank_progress_bar.reset();

    let chunk_size = number_of_tank_combinations / number_of_threads;

    // Copy all tanks from combination_map into a Vector, only retaining the first player of each duo
    let clean_tank_vec: Vec<Vec<u8>> = combination_map[&Position::Tank]
        .iter()
        .map(|c| c.iter().map(|d| d.0).collect())
        .collect();

    let tank_chunks = clean_tank_vec.into_iter().chunks(chunk_size);

    let mut handles = vec![];

    let mutex = Mutex::new((matchup_vec, tank_progress_bar));
    let arc = Arc::new(mutex);

    for tank_chunk in tank_chunks.into_iter() {
        let tank_chunk: Vec<Vec<u8>> = tank_chunk.collect();

        let dps_supp_combinations = all_dps_supp_combinations.clone();

        let cloned_arc = Arc::clone(&arc);

        let handle = thread::spawn(move || {
            let mut matchups_this_thread: Vec<(Vec<u8>, Vec<(u8, u8)>, Vec<(u8, u8)>)> = Vec::new();
            let mut counter = 0;

            for tank_pairs_in_this_matchup in tank_chunk {
                let mut tank_names: Vec<u8> = Vec::new();
                for tank in &tank_pairs_in_this_matchup {
                    tank_names.push(*tank);
                }

                'dps_support_loop: for dps_support_pairs in &dps_supp_combinations {
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
                    if names_in_this_matchup.iter().unique().count() != names_in_this_matchup.len()
                    {
                        continue 'dps_support_loop;
                    }

                    // No names are double, this is a possibly valid matchup
                    let dps_pairs: Vec<(u8, u8)> = dps_support_pairs.0.clone();
                    let support_pairs: Vec<(u8, u8)> = dps_support_pairs.1.clone();

                    let full_matchup: (Vec<u8>, Vec<(u8, u8)>, Vec<(u8, u8)>) =
                        (tank_pairs_in_this_matchup.clone(), dps_pairs, support_pairs);

                    matchups_this_thread.push(full_matchup);
                }
                counter += 1;
                if (counter % 10) == 0 {
                    cloned_arc.lock().unwrap().1.inc(10);
                }
            }
            cloned_arc.lock().unwrap().0.push(matchups_this_thread);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let matchup_vector = arc.lock().unwrap().0.clone();
    let mut matchups: Vec<(Vec<u8>, Vec<(u8, u8)>, Vec<(u8, u8)>)> = Vec::new();
    for mut matchup_chunk in matchup_vector {
        matchups.append(&mut matchup_chunk);
    }

    // println!("There are {} possible matchups", matchups.len());

    matchups
}
