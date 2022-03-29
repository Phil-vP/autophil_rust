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

use std::collections::HashMap;

fn main() {
    let player_vec: Vec<Player> = read_players();
    for player in player_vec.clone() {
        // println!("{}", player.to_string());
    }

    make_teams(player_vec);
}

// Read all players from players.txt
fn read_players() -> Vec<Player> {
    // open file players.txt and read all lines that don't start with #
    let mut players: Vec<Player> = Vec::new();
    let mut file = File::open("players.txt").expect("File not found");
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

fn make_teams(players: Vec<Player>) -> (HashMap<String, Vec<Team>>, Vec<Player>) {
    let mut teams: Vec<Team> = Vec::new();
    let mut players_left: Vec<Player> = Vec::new();

    let mut team_names: Vec<String> = vec!["Naughty Tomatoes".to_string()];

    let positions = [
        Position::Tank,
        Position::Tank,
        Position::Damage,
        Position::Damage,
        Position::Support,
        Position::Support,
    ];

    let number_of_teams: i16 = players.len() as i16 / 6;
    for _ in (team_names.len() as i16)..number_of_teams {
        let team_name = format!("Team {}", team_names.len() + 1);
        team_names.push(team_name);
    }

    team_names.reverse();

    println!("{:?}", team_names);

    let number_of_players_playing = number_of_teams * 6;
    let number_of_players = players.len();

    // let perms = (0..number_of_players).permutations(number_of_players_playing as usize);

    let mut possible_matchups: HashMap<String, Vec<Team>> = HashMap::new();

    // println!("{} permutations", perms.clone().count());

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
    for tank_pair in all_tank_pairs.clone() {
        println!("{:?}", tank_pair);
    }
    let all_damage_pairs = damage_list.iter().combinations(2);
    let all_support_pairs = support_list.iter().combinations(2);

    println!("There are {} tank pairs", all_tank_pairs.clone().count());
    println!("There are {} damage pairs", all_damage_pairs.clone().count());
    println!("There are {} support pairs", all_support_pairs.clone().count());


    let tank_combinations = all_tank_pairs.combinations(number_of_teams as usize);
    let damage_combinations = all_damage_pairs.combinations(number_of_teams as usize);
    let support_combinations = all_support_pairs.combinations(number_of_teams as usize);

    'tank_loop: for tank_pair in tank_combinations {
        let mut all_players_this_run: Vec<Player> = Vec::new();
        
        for damage_pair in damage_combinations.clone() {
            for support_pair in support_combinations.clone() {
                
            }
        }
    }

    (possible_matchups, players_left)
}



    /*
    'perm_loop: for perm in perms {
        let mut team_names_clone = team_names.clone();
        let mut teams: Vec<Team> = Vec::new();
        // Each permutation is a possible matchup
        // Check if the permutation is even valid
        let mut team_slices = perm.chunks(6);
        // println!("{:?}", team_slices);
        let mut team_hashes: Vec<String> = Vec::new();
        // For every "team" in this permutation
        for team_index in 0..team_slices.len() {
            // Check if every player is playing the respective position
            // println!("team_index: {}", team_index);
            let team_slice = team_slices.next().unwrap();
            for player_index in 0..6 {
                let player = players[team_slice[player_index] as usize].clone();
                if !player.plays_position(positions[player_index]) {
                    continue 'perm_loop;
                }
            }
            // Now we know that every player is playing the respective position
            // The team can now be filled and added to the teams vector
            teams.push(Team::new(
                team_names_clone.pop().unwrap(),
                players[team_slice[0] as usize].clone(),
                players[team_slice[1] as usize].clone(),
                players[team_slice[2] as usize].clone(),
                players[team_slice[3] as usize].clone(),
                players[team_slice[4] as usize].clone(),
                players[team_slice[5] as usize].clone(),
            ));
            // Add the team's hash to the team_hashes vector
            team_hashes.push(teams[team_index].get_ID().unwrap());
        }
        // Check if the team_hashes vector contains duplicates
        team_hashes.sort();
        let full_hash = team_hashes.join("");

        if !possible_matchups.contains_key(&full_hash) {
            possible_matchups.insert(full_hash, teams);
        }
    }

    println!("{} possible matchups", possible_matchups.len());

    /*
    for (hash, teams) in &possible_matchups {
        println!("{}", hash);
        for team in teams {
            println!("{}", team.to_full_String().unwrap());
        }
    }
    */

    (possible_matchups, players_left)
}
*/
