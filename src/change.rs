use crate::{Player, Team, Position};

pub struct Change {
    pub team_1: Team,
    pub team_2: Team,
    pub player_1: Player,
    pub player_2: Player,
    pub player_1_pos: Position,
    pub player_2_pos: Position,
    pub sr_change: i16,
}