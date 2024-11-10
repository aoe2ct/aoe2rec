use binrw::io::{BufReader, Cursor};
use binrw::{binrw, BinReaderExt, BinResult, NullString};
use serde::Serialize;
use std::error::Error;
use std::fs::File;

#[binrw]
pub struct EncodedHeader {
    length: u32,
    other: u32,
    #[br(count = length - 8)]
    zheader: Vec<u8>,
    log_version: u32,
}

#[binrw]
#[derive(Serialize)]
#[br(import(major: u16))]
pub struct GameSettings {
    game_options_version: u32,
    n_dlc: u32,
    #[br(count=n_dlc)]
    dlcs: Vec<u32>,
    dataset_ref: u32,
    difficulty: u32,
    selected_map_id: u32,
    resolved_map_id: u32,
    reveal_map: u32,
    victory_type_id: u32,
    starting_resources_id: u32,
    starting_age_id: u32,
    ending_age_id: u32,
    game_type: u32,
    #[br(magic = b"\xa3_\x02\x00\xa3_\x02\x00")]
    speed: f32,
    treaty_length: u32,
    population_limit: u32,
    n_players: u32,
    unused_player_color: u32,
    victory_amount: i32,
    unknown_field: u8,
    #[br(magic = b"\xa3_\x02\x00")]
    trade_enabled: Bool,
    team_bonus_disabled: Bool,
    random_positions: Bool,
    all_techs: Bool,
    num_starting_units: u8,
    lock_teams: Bool,
    lock_speed: Bool,
    multiplayer: Bool,
    cheats: Bool,
    record_game: Bool,
    animals_enabled: Bool,
    predators_enables: Bool,
    turbo_enabled: Bool,
    shared_exploration: Bool,
    team_positions: Bool,
    sub_game_mode: u32,
    battle_royale_time: u32,
    handicap: Bool,
    unk: Bool,
    #[br(magic = b"\xa3_\x02\x00")]
    #[br(count = n_players)]
    players: Vec<Player>,
    unknown: [u8; 9],
    fog_of_war: Bool,
    cheat_notifications: Bool,
    colored_chat: Bool,
    #[serde(skip_serializing)]
    #[br(count = 8 - n_players)]
    empty_slots: Vec<EmptySlot>,
    #[br(magic = b"\xa3_\x02\x00")]
    ranked: Bool,
    allow_specs: Bool,
    lobby_visibility: u32,
    hidden_civs: Bool,
    matchmaking: Bool,
    spec_delay: u32,
    scenario_civ: Bool,
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    rms_strings: Vec<DeString>,
    #[serde(skip_serializing)]
    unknown2: u32,
    #[serde(skip_serializing)]
    unknown3: u32,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings2: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings3: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings4: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings5: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings6: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings7: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings8: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings9: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings10: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings11: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings12: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings13: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings14: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings15: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings16: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings17: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings18: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings19: Vec<DeString>,
    #[serde(skip_serializing)]
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings20: Vec<DeString>,
    #[serde(skip_serializing)]
    num_strategic_numbers: u32,
    #[br(count = num_strategic_numbers)]
    strategic_numbers: Vec<i32>,
    num_ai_files: u32,
    #[br(count = num_ai_files)]
    ai_files: Vec<AIFile>,
    #[serde(skip_serializing)]
    unknown4: u32, // 25.02
    #[serde(skip_serializing)]
    unknown5: u32, // 25.02
    #[serde(skip_serializing)]
    unknown6: u32, // 25.02
    guid: [u8; 16],
    lobby_name: DeString,
    #[serde(skip_serializing)]
    unknown7: [u8; 8], // 25.22
    modded_dataset: DeString,
    #[serde(skip_serializing)]
    unknown8: [u8; 19],
    #[serde(skip_serializing)]
    unknown9: [u8; 5], // 13.13
    #[serde(skip_serializing)]
    unknown10: [u8; 3], // 13.17
    #[serde(skip_serializing)]
    unknown11: DeString, // 13.17
    #[serde(skip_serializing)]
    unknown12: [u8; 3], // 13.17
    #[serde(skip_serializing)]
    unknown13: u8, // 20.06
    #[serde(skip_serializing)]
    unknown14: [u8; 8], // 20.16
    #[serde(skip_serializing)]
    unknown15: [u8; 21], // 25.06
    #[serde(skip_serializing)]
    unknown16: [u8; 4], // 25.22
    #[serde(skip_serializing)]
    unknown17: [u8; 8], // 26.16
    #[serde(skip_serializing)]
    unknown18: [u8; 3], // 37
    #[serde(skip_serializing)]
    unknown19: [u8; 8], // 50
    #[serde(skip_serializing)]
    #[br(if(major >= 63))]
    unknown24: Option<[u8; 5]>,
    #[serde(skip_serializing)]
    unknown20: DeString,
    #[serde(skip_serializing)]
    unknown21: [u8; 5],
    #[serde(skip_serializing)]
    unknown22: u8, // 13.13
    #[serde(skip_serializing)]
    unknown23: [u8; 2], // 13.13
    ver37: [u32; 2],
}

#[binrw]
#[derive(Serialize)]
pub struct AIConfig {
    has_ai: u32,
    // TODO: Implement the case where the AI is present
}

#[binrw]
#[derive(Serialize)]
pub struct Replay {
    old_time: u32,
    world_time: u32,
    old_world_time: u32,
    game_speed_id: u32,
    world_time_delta_seconds: u32,
    timer: f32,
    game_speed: f32,
    temp_pause: Bool,
    next_object_id: u32,
    next_reusable_object_id: i32,
    random_seed: u32,
    random_seed_2: u32,
    rec_player: u16,
    num_players: u8,
    instant_build: Bool,
    cheats_enabled: Bool,
    game_mode: u16,
    campaign: u32,
    campaign_player: u32,
    campaign_scenario: u32,
    king_campaign: u32,
    king_campaign_player: u8,
    king_campaign_scenario: u8,
    player_turn: u32,
    #[br(count = num_players)]
    player_turns: Vec<u32>,
    #[serde(skip_serializing)]
    padding: [u8; 8],
}

#[binrw]
#[derive(Serialize)]
pub struct RecHeader {
    game: MyNullString,
    save: f32,
    version_minor: u16,
    version_major: u16,
    build: u32,
    timestamp: i32,
    version_2: [u16; 2],
    interval_version: [u16; 2],
    #[br(args(version_major))]
    game_settings: GameSettings,
    ai_config: AIConfig,
    replay: Replay,
    map_info: MapInfo,
}

#[binrw]
#[derive(Serialize)]
#[br(import(tile_count: u32))]
pub struct IgnoreMapTile {
    tile_num: u32,
    #[serde(skip_serializing)]
    unknown1: [u8; 2044],
    #[serde(skip_serializing)]
    #[br(count = tile_count)]
    unknown_tiles: Vec<u16>,
    float_count: u32,
    #[serde(skip_serializing)]
    #[br(count = float_count)]
    unknown2: Vec<f32>,
    #[serde(skip_serializing)]
    unknown3: u32,
}

#[binrw]
#[derive(Serialize)]
pub struct MapInfo {
    size_x: u32,
    size_y: u32,
    zone_count: u32,
    #[br(args { inner: (size_x * size_y,)} )]
    #[br(count = zone_count)]
    ignored_map_tiles: Vec<IgnoreMapTile>,
    all_visible: Bool,
    fog_of_war: Bool,
}

#[binrw]
#[derive(Serialize)]
pub struct Player {
    dlc_id: u32,
    color_id: i32,
    selected_color: u8,
    selected_team_id: u8,
    resolved_team_id: u8,
    #[serde(skip_serializing)]
    dat_crc: [u8; 8],
    mp_game_version: u8,
    civ_id: u32,
    custom_civ_count: u32,
    #[br(count = custom_civ_count)]
    custom_civ_ids: Vec<u32>,
    ai_type: DeString,
    ai_civ_name_index: u8,
    ai_name: DeString,
    name: DeString,
    player_type: u32,
    profile_id: u32,
    ai: [u8; 4],
    player_number: i32,
    prefer_random: u8,
    custom_ai: u8,
    handicap: [u8; 8],
}

#[binrw]
#[derive(Serialize)]
struct EmptySlot {
    i0x: u32,
    i0a: u32,
    i0b: u32,
    unknown2: u32,
    s1: DeString,
    a2: u8,
    s2: DeString,
    s3: DeString,
    a3: [u8; 22],
    i1: u32,
    i2: u32,
    a4: [u8; 8],
}

#[binrw]
#[derive(Serialize)]
struct AIFile {
    unknown: u32,
    name: DeString,
    unknown2: u32,
}

#[binrw]
struct Bool {
    #[br(map = |x: u8| x == 1)]
    #[bw(map = |ranked: &bool| match ranked { true => 1u8, false => 0u8})]
    value: bool,
}

impl std::fmt::Debug for Bool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Serialize for Bool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bool(self.value)
    }
}

#[binrw]
pub struct DeString {
    #[br(magic = b"\x60\x0A")]
    length: u16,
    #[br(count = length)]
    value: Vec<u8>,
}

#[binrw]
pub struct MyNullString {
    text: NullString,
}

impl std::fmt::Debug for DeString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::string::String::from_utf8_lossy(&self.value))
    }
}

impl Serialize for DeString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let strvalue = std::string::String::from_utf8_lossy(&self.value);
        serializer.serialize_str(&strvalue)
    }
}

impl serde::Serialize for MyNullString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let strvalue = std::string::String::from_utf8_lossy(&self.text);
        serializer.serialize_str(&strvalue)
    }
}

impl RecHeader {
    pub fn build<T: BinReaderExt>(mut reader: T) -> Result<RecHeader, Box<dyn Error>> {
        let encoded_header: EncodedHeader = reader.read_le()?;
        let (header, _) = yazi::decompress(&encoded_header.zheader, yazi::Format::Raw).unwrap();
        let mut hreader = Cursor::new(header);
        let parsed_header: RecHeader = hreader.read_le()?;

        Ok(parsed_header)
    }
    pub fn from_bytes(data: bytes::Bytes) -> Result<RecHeader, Box<dyn Error>> {
        let breader = BufReader::new(Cursor::new(data));
        return RecHeader::build(breader);
    }
    pub fn from_file(path: &std::path::Path) -> Result<RecHeader, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        return RecHeader::build(reader);
    }
}

#[binrw::parser(reader, endian)]
fn read_strings_of_length() -> BinResult<Vec<DeString>> {
    let mut strings: Vec<DeString> = Vec::new();
    loop {
        let crc: u32 = reader.read_type(endian)?;
        if crc > 0 && crc < 255 {
            break;
        }
        let string: DeString = reader.read_type(endian)?;
        strings.push(string);
    }
    Ok(strings)
}

#[binrw::writer()]
fn write_len_and_string(_strings: &Vec<DeString>) -> BinResult<()> {
    // for string in strings {
    //     writer.write_type(&u32::try_from(string.len()).unwrap(), endian)?;
    //     writer.write_type(&string, endian)?;
    // }
    // writer.write_type(&0u32, endian)?;
    Ok(())
}
