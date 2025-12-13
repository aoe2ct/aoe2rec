mod ai;
mod map;

use crate::{
    read_strings_of_length, write_len_and_string,
    primitives::{Bool, DeString, LenString16, MyNullString}
};
use ai::{AIFile, AIInfo};
use binrw::io::{BufReader, NoSeek};
use binrw::{binrw, BinReaderExt};
use map::MapInfo;
use serde::Serialize;

pub fn decompress(header_data: Vec<u8>) -> ChapterHeader {
    let reader = flate2::read::DeflateDecoder::new(&header_data[..]);
    let parsed_header: ChapterHeader = BufReader::new(NoSeek::new(reader)).read_le().unwrap();

    parsed_header
}

#[binrw]
#[derive(Serialize)]
pub struct ChapterHeader {
    pub game: MyNullString,
    pub save: f32,
    #[br(map = |x: u16| (x as f32 / 6553.6).round() as u16)]
    pub version_minor: u16,
    pub version_major: u16,
    pub build: u32,
    pub timestamp: i32,
    pub version_2: [u16; 2],
    pub interval_version: [u16; 2],
    #[br(args(version_major))]
    pub game_settings: GameSettings,
    pub ai_config: AIConfig,
    pub replay: Replay,
    pub map_info: MapInfo,
    #[br(args(replay.num_players, version_major))]
    pub initial: Initial,
}

#[binrw]
#[derive(Serialize, Debug)]
#[br(import(major: u16))]
pub struct GameSettings {
    pub game_options_version: u32,
    #[bw(calc(dlcs.len().try_into().unwrap()))]
    pub n_dlc: u32,
    #[br(count=n_dlc)]
    pub dlcs: Vec<u32>,
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
    #[br(magic = b"\xa3_\x02\x00\xa3_\x02\x00")]
    #[bw(magic = b"\xa3_\x02\x00\xa3_\x02\x00")]
    pub speed: f32,
    pub treaty_length: u32,
    pub population_limit: u32,
    // #[bw(calc(players.len().try_into().unwrap()))]
    pub n_players: u32,
    pub unused_player_color: u32,
    pub victory_amount: i32,
    pub unknown_field: u8,
    #[br(magic = b"\xa3_\x02\x00")]
    #[bw(magic = b"\xa3_\x02\x00")]
    pub trade_enabled: Bool,
    pub team_bonus_disabled: Bool,
    pub random_positions: Bool,
    pub all_techs: Bool,
    pub num_starting_units: u8,
    pub lock_teams: Bool,
    pub lock_speed: Bool,
    pub multiplayer: Bool,
    pub cheats: Bool,
    pub record_game: Bool,
    pub animals_enabled: Bool,
    pub predators_enables: Bool,
    pub turbo_enabled: Bool,
    pub shared_exploration: Bool,
    pub team_positions: Bool,
    pub sub_game_mode: u32,
    pub battle_royale_time: u32,
    pub handicap: Bool,
    pub unk: Bool,
    #[br(magic = b"\xa3_\x02\x00")]
    #[bw(magic = b"\xa3_\x02\x00")]
    #[br(count = n_players, args { inner: (major,)})]
    pub players: Vec<Player>,
    #[br(if(major >= 66), count = 8 - n_players, args { inner: (major,)})]
    pub empty_player_slots: Vec<Player>,
    pub unknown: [u8; 9],
    pub fog_of_war: Bool,
    pub cheat_notifications: Bool,
    pub colored_chat: Bool,
    #[serde(skip_serializing)]
    #[br(if(major < 66), count = 8 - n_players, args { inner: (major,)})]
    pub empty_slots: Vec<EmptySlot>,
    #[br(magic = b"\xa3_\x02\x00")]
    #[bw(magic = b"\xa3_\x02\x00")]
    pub ranked: Bool,
    pub allow_specs: Bool,
    pub lobby_visibility: u32,
    pub hidden_civs: Bool,
    pub matchmaking: Bool,
    pub spec_delay: u32,
    pub scenario_civ: Bool,
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    pub rms_strings: Vec<DeString>,
    #[serde(skip_serializing)]
    pub unknown2: u32,
    #[serde(skip_serializing)]
    pub unknown3: u32,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    pub other_strings: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    pub other_strings2: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    pub other_strings3: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    pub other_strings4: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    pub other_strings5: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    pub other_strings6: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    pub other_strings7: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    pub other_strings8: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    pub other_strings9: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    pub other_strings10: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    pub other_strings11: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    pub other_strings12: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    pub other_strings13: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    pub other_strings14: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    pub other_strings15: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    pub other_strings16: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    pub other_strings17: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    pub other_strings18: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    pub other_strings19: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    pub other_strings20: Vec<DeString>,
    #[serde(skip_serializing)]
    pub num_strategic_numbers: u32,
    #[br(count = num_strategic_numbers)]
    pub strategic_numbers: Vec<i32>,
    pub num_ai_files: u32,
    #[br(count = num_ai_files)]
    pub ai_files: Vec<AIFile>,
    #[serde(skip_serializing)]
    pub unknown4: u32, // 25.02
    #[serde(skip_serializing)]
    pub unknown5: u32, // 25.02
    #[serde(skip_serializing)]
    pub unknown6: u32, // 25.02
    pub guid: [u8; 16],
    pub lobby_name: DeString,
    #[serde(skip_serializing)]
    pub unknown7: [u8; 8], // 25.22
    pub modded_dataset: DeString,
    #[serde(skip_serializing)]
    pub unknown8: [u8; 19],
    #[serde(skip_serializing)]
    pub unknown9: [u8; 5], // 13.13
    #[serde(skip_serializing)]
    pub unknown10: [u8; 3], // 13.17
    #[serde(skip_serializing)]
    pub unknown11: DeString, // 13.17
    #[serde(skip_serializing)]
    pub unknown12: [u8; 3], // 13.17
    #[serde(skip_serializing)]
    pub unknown13: u8, // 20.06
    #[serde(skip_serializing)]
    pub unknown14: [u8; 8], // 20.16
    #[serde(skip_serializing)]
    pub unknown15: [u8; 21], // 25.06
    #[serde(skip_serializing)]
    pub unknown16: [u8; 4], // 25.22
    #[serde(skip_serializing)]
    pub unknown17: [u8; 8], // 26.16
    #[serde(skip_serializing)]
    pub unknown18: [u8; 3], // 37
    #[serde(skip_serializing)]
    pub unknown19: [u8; 8], // 50
    #[serde(skip_serializing)]
    #[br(if(major >= 63))]
    pub unknown24: Option<[u8; 5]>,
    #[br(if(major >= 66))]
    pub unknown_count1: u32,
    #[br(if(major >= 66))]
    pub unknown26: Option<[u8; 12]>,
    #[br(count = unknown_count1)]
    pub unknown27: Vec<u32>,
    #[serde(skip_serializing)]
    pub unknown20: DeString,
    #[serde(skip_serializing)]
    pub unknown21: [u8; 5],
    #[serde(skip_serializing)]
    pub unknown22: u8, // 13.13
    #[serde(skip_serializing)]
    pub unknown23: [u8; 2], // 13.13
    pub timestamp: u32,
    #[serde(skip_serializing)]
    pub unknown25: u32,
}

#[binrw]
#[derive(Serialize, Debug)]
pub enum AIConfig {
    #[br(magic = 1u32)]
    WithAI(AIInfo),
    #[br(magic = 0u32)]
    WithoutAI {},
}

#[binrw]
#[derive(Serialize, Debug)]
pub struct Replay {
    pub old_time: u32,
    pub world_time: u32,
    pub old_world_time: u32,
    pub game_speed_id: u32,
    pub world_time_delta_seconds: u32,
    pub timer: f32,
    pub game_speed: f32,
    pub temp_pause: Bool,
    pub next_object_id: u32,
    pub next_reusable_object_id: i32,
    pub random_seed: u32,
    pub random_seed_2: u32,
    pub rec_player: u16,
    pub num_players: u8,
    pub instant_build: Bool,
    pub cheats_enabled: Bool,
    pub game_mode: u16,
    pub campaign: u32,
    pub campaign_player: u32,
    pub campaign_scenario: u32,
    pub king_campaign: u32,
    pub king_campaign_player: u8,
    pub king_campaign_scenario: u8,
    pub player_turn: u32,
    #[br(count = num_players)]
    pub player_turns: Vec<u32>,
    #[serde(skip_serializing)]
    pub padding: [u8; 8],
}

#[binrw]
#[derive(Serialize, Debug)]
#[br(import(major: u16))]
pub struct Player {
    pub dlc_id: u32,
    pub color_id: i32,
    pub selected_color: u8,
    pub selected_team_id: u8,
    pub resolved_team_id: u8,
    #[serde(skip_serializing)]
    pub dat_crc: [u8; 8],
    pub mp_game_version: u8,
    pub civ_id: u32,
    pub custom_civ_count: u32,
    #[br(count = custom_civ_count)]
    pub custom_civ_ids: Vec<u32>,
    pub ai_type: DeString,
    pub ai_civ_name_index: u8,
    pub ai_name: DeString,
    #[br(if(major >= 66))]
    pub censored_name: DeString,
    pub name: DeString,
    pub player_type: u32,
    pub profile_id: i32,
    pub ai: [u8; 4],
    pub player_number: i32,
    pub prefer_random: Bool,
    pub custom_ai: u8,
    pub handicap: [u8; 8],
    #[br(if(major >= 64))]
    pub unknown_de_64_19661: u32,
}

#[binrw]
#[derive(Serialize, Debug)]
#[br(import(major: u16))]
pub struct EmptySlot {
    pub i0x: u32,
    pub i0a: u32,
    pub i0b: u32,
    pub unknown2: u32,
    pub s1: DeString,
    pub a2: u8,
    pub s2: DeString,
    pub s3: DeString,
    pub a3: [u8; 22],
    pub i1: u32,
    pub i2: u32,
    pub a4: [u8; 8],
    #[br(if(major >= 64))]
    pub unknown_de_64_19661: u32,
}

#[binrw]
#[derive(Serialize, Debug)]
#[br(import(num_players: u8, major: u16))]
pub struct Initial {
    pub restore_time: u32,
    pub num_particles: u32,
    #[serde(skip_serializing)]
    #[br(count = num_particles * 27)]
    pub particles: Vec<u8>,
    pub identifier: u32,
    #[serde(skip_serializing)]
    #[br(count = 1, args { inner: (num_players, major) })]
    pub players: Vec<PlayerInit>,
    #[serde(skip_serializing)]
    pub unknown1: [u8; 21],
}
#[binrw]
#[derive(Serialize, Debug, Default)]
pub struct InnerUnknownPlayerStruct2 {
    unknown_count: u16,
    #[br(count = unknown_count * 13)]
    unknown1: Vec<u8>,
    unknown2: u8,
}

#[binrw]
#[derive(Serialize, Debug)]
#[br(import(major: u16))]
pub struct InnerUnknownPlayerStruct {
    pub unknown_type: u16,
    pub unknown1: DeString,
    pub unknown2: DeString,
    pub unknown3: [u16; 16],
    #[br(if(major >= 66))]
    pub unknown5: InnerUnknownPlayerStruct2,
    #[br(if(major < 66))]
    pub unknown6: [u16; 8],
    #[br(if(major >= 64))]
    pub unknown4: u16,
    #[br(if(major >= 66))]
    pub unknown7: [u8; 12],
}

#[binrw]
#[derive(Serialize, Debug)]
#[br(import(major: u16))]
pub struct UnknownPlayerStruct {
    pub sub_count: u16,
    pub unknown1: [u32; 2],
    pub unknown2: [u8; 5],
    #[br(if(major < 66))]
    pub unknown3: [u8; 5],
    #[br(count=sub_count, args { inner: (major,) })]
    pub unknown_inner: Vec<InnerUnknownPlayerStruct>,
}

#[binrw]
#[derive(Serialize, Debug)]
#[br(import(num_players: u8, major: u16))]
pub struct PlayerInit {
    pub player_type: u8,
    #[serde(skip_serializing)]
    pub unknown1: u8,
    #[serde(skip_serializing)]
    #[br(count = num_players)]
    pub their_diplomacy: Vec<u8>,
    #[br(count = num_players)]
    pub own_diplomacy: Vec<i32>,
    pub allied_los: u32,
    pub allied_victory: Bool,
    pub player_name: LenString16,

    //#[br(magic = b"\x16")]
    unk1: u8,

    pub header_data_count: i32,

    //#[br(magic = b"\x21")]
    unk2: u8,

    #[br(count=header_data_count)]
    pub player_stats: Vec<f32>,
    #[br(count=header_data_count)]
    pub extra_player_stats: Vec<f32>,
    pub padding: u8,
    pub view: PlayerView,
    pub saved_views_count: i32,
    #[br(count = if saved_views_count < 0 { 0 } else { saved_views_count })]
    pub saved_views: Vec<PlayerView>,
    pub spawn_location: Location,
    pub culture: u8,
    pub civilization: u8,
    pub game_status: u8,
    //#[br(dbg, pad_after = 1)]
    pub resigned: Bool,
    unk5: u8,

    pub player_color: u8,
    unk6: u8,

    #[serde(skip_serializing)]
    //#[br(magic = b"\x00\x0B")]
    unk3: u16,

    pub unknown3: u8,

    #[serde(skip_serializing)]
    //#[br(magic = b"\x0B")]
    unk4: u8,

    pub unknown4: [u32; 8],
    #[serde(skip_serializing)]
    pub unknown5: f32,
    #[serde(skip_serializing)]
    #[br(count = 197)]
    pub unknown6: Vec<u8>,
    #[serde(skip_serializing)]
    #[br(args(major))]
    pub unknown_struct: UnknownPlayerStruct,
    pub dev: [u32; 20],  // TODO: Finish implementing
    pub dev1: [u32; 20], // TODO: Finish implementing
    pub dev2: [u32; 20], // TODO: Finish implementing
    pub dev3: [i32; 20], // TODO: Finish implementing
    pub dev4: [i32; 20], // TODO: Finish implementing
    pub dev5: [i32; 20], // TODO: Finish implementing
    pub dev6: [i32; 20], // TODO: Finish implementing
    pub dev7: [i32; 20], // TODO: Finish implementing
}

#[binrw]
#[derive(Serialize, Debug)]
pub struct PlayerView {
    pub camera_x: f32,
    pub camera_y: f32,
}

#[binrw]
#[derive(Serialize, Debug)]
pub struct Location {
    pub x: u16,
    pub y: u16,
}
