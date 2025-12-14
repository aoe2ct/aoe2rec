mod utils;

use aoe2rec::Savegame;
use bytes::Bytes;

use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn parse_rec(buffer: js_sys::ArrayBuffer) -> JsValue {
    utils::set_panic_hook();

    let input_rec = Uint8Array::new(&buffer).to_vec();
    let rec = Savegame::from_bytes(&Bytes::from(input_rec)).unwrap();
    serde_wasm_bindgen::to_value(&rec).unwrap()
}

#[wasm_bindgen]
pub fn parse_rec_summary(buffer: js_sys::ArrayBuffer) -> SavegameSummary {
    utils::set_panic_hook();

    let input_rec = Uint8Array::new(&buffer).to_vec();
    let rec = Savegame::from_bytes(&Bytes::from(input_rec)).unwrap();
    SavegameSummary::from(rec.get_summary())
}

#[wasm_bindgen(getter_with_clone)]
pub struct SavegameSummary {
    pub header: SavegameHeader,
    pub teams: Vec<Team>,
    pub duration: u32,
}

impl From<aoe2rec::summary::SavegameSummary<'_>> for SavegameSummary {
    fn from(summary: aoe2rec::summary::SavegameSummary) -> SavegameSummary {
        SavegameSummary {
            header: SavegameHeader {
                game_string: summary.header.game.to_owned().into(),
                version_major: summary.header.version_major,
                version_minor: summary.header.version_minor,
                build: summary.header.build,
                timestamp: summary.header.timestamp,
                game_settings: GameSettings::from(&summary),
                replay: Replay::from(&summary),
            },
            teams: summary.teams.iter().map(|team| team.into()).collect(),
            duration: summary.duration,
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct SavegameHeader {
    pub game_string: String,
    pub version_major: u16,
    pub version_minor: u16,
    pub build: u32,
    pub timestamp: i32,
    pub game_settings: GameSettings,
    pub replay: Replay,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct GameSettings {
    pub dataset_ref: u32,
    pub difficulty: u32,
    pub selected_map_id: u32,
    pub resolved_map_id: u32,
    pub reveal_map: u32,
    pub victory_type_id: u32,
    pub starting_resources_id: u32,
    pub starting_age_id: u32,
    pub ending_age_id: u32,
    pub game_type: u32,
    pub speed: f32,
    pub treaty_length: u32,
    pub population_limit: u32,
    pub n_players: u32,
    pub victory_amount: i32,
    pub trade_enabled: bool,
    pub team_bonus_disabled: bool,
    pub random_positions: bool,
    pub all_techs: bool,
    pub num_starting_units: u8,
    pub lock_teams: bool,
    pub lock_speed: bool,
    pub multiplayer: bool,
    pub cheats: bool,
    pub record_game: bool,
    pub shared_exploration: bool,
    pub team_positions: bool,
    pub sub_game_mode: u32,
    pub battle_royale_time: u32,
    pub handicap: bool,
    pub fog_of_war: bool,
    pub ranked: bool,
    pub allow_specs: bool,
    pub lobby_visibility: u32,
    pub hidden_civs: bool,
    pub matchmaking: bool,
    pub spec_delay: u32,
    pub scenario_civ: bool,
    pub rms_strings: Vec<String>,
    pub lobby_name: String,
    pub modded_dataset: String,
}

impl From<&aoe2rec::summary::SavegameSummary<'_>> for GameSettings {
    fn from(summary: &aoe2rec::summary::SavegameSummary) -> GameSettings {
        let gs = summary.header.game_settings;
        let map_strings: Vec<String> = gs
            .rms_strings
            .iter()
            .map(|map_str| map_str.clone().into())
            .collect();
        GameSettings {
            dataset_ref: gs.dataset_ref,
            difficulty: gs.difficulty,
            selected_map_id: gs.selected_map_id,
            resolved_map_id: gs.resolved_map_id,
            reveal_map: gs.reveal_map,
            victory_type_id: gs.victory_type_id,
            starting_resources_id: gs.starting_resources_id,
            starting_age_id: gs.starting_age_id,
            ending_age_id: gs.ending_age_id,
            game_type: gs.game_type,
            speed: gs.speed,
            treaty_length: gs.treaty_length,
            population_limit: gs.population_limit,
            n_players: gs.n_players,
            victory_amount: gs.victory_amount,
            trade_enabled: gs.trade_enabled.into(),
            team_bonus_disabled: gs.team_bonus_disabled.into(),
            random_positions: gs.random_positions.into(),
            all_techs: gs.all_techs.into(),
            num_starting_units: gs.num_starting_units,
            lock_teams: gs.lock_teams.into(),
            lock_speed: gs.lock_speed.into(),
            multiplayer: gs.multiplayer.into(),
            cheats: gs.cheats.into(),
            record_game: gs.record_game.into(),
            shared_exploration: gs.shared_exploration.into(),
            team_positions: gs.team_positions.into(),
            sub_game_mode: gs.sub_game_mode,
            battle_royale_time: gs.battle_royale_time,
            handicap: gs.handicap.into(),
            fog_of_war: gs.fog_of_war.into(),
            ranked: gs.ranked.into(),
            allow_specs: gs.allow_specs.into(),
            lobby_visibility: gs.lobby_visibility,
            hidden_civs: gs.hidden_civs.into(),
            matchmaking: gs.matchmaking.into(),
            spec_delay: gs.spec_delay,
            scenario_civ: gs.scenario_civ.into(),
            rms_strings: map_strings,
            lobby_name: gs.lobby_name.clone().into(),
            modded_dataset: gs.modded_dataset.clone().into(),
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct Replay {
    pub old_time: u32,
    pub world_time: u32,
    pub old_world_time: u32,
    pub game_speed_id: u32,
    pub world_time_delta_seconds: u32,
    pub timer: f32,
    pub game_speed: f32,
    pub temp_pause: bool,
    pub random_seed: u32,
    pub random_seed_2: u32,
    pub rec_player: u16,
    pub num_players: u8,
    pub instant_build: bool,
    pub cheats_enabled: bool,
    pub game_mode: u16,
}

impl From<&aoe2rec::summary::SavegameSummary<'_>> for Replay {
    fn from(summary: &aoe2rec::summary::SavegameSummary) -> Replay {
        let r = summary.header.replay;
        Replay {
            old_time: r.old_time,
            world_time: r.world_time,
            old_world_time: r.old_world_time,
            game_speed_id: r.game_speed_id,
            world_time_delta_seconds: r.world_time_delta_seconds,
            timer: r.timer,
            game_speed: r.game_speed,
            temp_pause: r.temp_pause.into(),
            random_seed: r.random_seed,
            random_seed_2: r.random_seed_2,
            rec_player: r.rec_player,
            num_players: r.num_players,
            instant_build: r.instant_build.into(),
            cheats_enabled: r.cheats_enabled.into(),
            game_mode: r.game_mode,
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct Team {
    pub winner: bool,
    pub players: Vec<Player>,
}

impl From<&aoe2rec::summary::GameTeam<'_>> for Team {
    fn from(team: &aoe2rec::summary::GameTeam<'_>) -> Self {
        Self {
            winner: team.players.iter().any(|player| !player.resigned),
            players: team.players.iter().map(|player| player.into()).collect(),
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct Player {
    pub name: String,
    pub color_id: i32,
    pub selected_color: u8,
    pub selected_team_id: u8,
    pub resolved_team_id: u8,
    pub civ_id: u32,
    pub custom_civ_ids: Vec<u32>,
    pub player_type: u32,
    pub profile_id: i32,
    pub player_number: i32,
    pub prefer_random: bool,
    pub resigned: bool,
}

impl From<&aoe2rec::summary::TeamPlayer<'_>> for Player {
    fn from(player: &aoe2rec::summary::TeamPlayer<'_>) -> Self {
        Self {
            name: player.info.name.clone().into(),
            color_id: player.info.color_id,
            selected_color: player.info.selected_color,
            selected_team_id: player.info.selected_team_id,
            resolved_team_id: player.info.resolved_team_id,
            civ_id: player.info.civ_id,
            custom_civ_ids: player.info.custom_civ_ids.clone(),
            player_type: player.info.player_type,
            profile_id: player.info.profile_id,
            player_number: player.info.player_number,
            prefer_random: player.info.prefer_random.into(),
            resigned: player.resigned,
        }
    }
}
