use std::collections::BTreeSet;

use serde::Serialize;

use crate::{
    header::{GameSettings, Player, Replay},
    primitives::MyNullString,
    Savegame,
};

#[derive(Serialize, Debug)]
pub struct SavegameSummary<'a> {
    pub header: SummaryHeader<'a>,
    pub duration: u32,
    pub resignations: Vec<u8>,
    pub teams: Vec<GameTeam<'a>>,
}

#[derive(Serialize, Debug)]
pub struct SummaryHeader<'a> {
    pub game: &'a MyNullString,
    pub version_minor: u16,
    pub version_major: u16,
    pub build: u32,
    pub timestamp: i32,
    pub game_settings: &'a GameSettings,
    pub replay: &'a Replay,
}

#[derive(Serialize, Debug)]
pub struct GameTeam<'a> {
    pub players: Vec<TeamPlayer<'a>>,
}

#[derive(Serialize, Debug)]
pub struct TeamPlayer<'a> {
    pub resigned: bool,
    pub info: &'a Player,
}

impl<'a> GameTeam<'_> {
    pub fn from_savegame(savegame: &'a Savegame) -> Vec<GameTeam<'a>> {
        let resignations = savegame.get_resignations();
        let mut teams_ids = BTreeSet::new();
        for (index, player) in savegame.chapters[0]
            .header
            .game_settings
            .players
            .iter()
            .enumerate()
        {
            let team_id = match player.resolved_team_id {
                1 => (9 + index).try_into().unwrap_or(9),
                _ => player.resolved_team_id,
            };
            teams_ids.insert(team_id);
        }

        teams_ids
            .iter()
            .map(|team_id| GameTeam {
                players: savegame.chapters[0]
                    .header
                    .game_settings
                    .players
                    .iter()
                    .enumerate()
                    .filter(|(index, player)| match player.resolved_team_id {
                        1 => (9 + index).try_into().unwrap_or(9) == *team_id,
                        _ => player.resolved_team_id == *team_id,
                    })
                    .map(|(_, player)| TeamPlayer {
                        resigned: resignations
                            .contains(&player.player_number.try_into().unwrap_or(100)),
                        info: player,
                    })
                    .collect(),
            })
            .collect()
    }
}
