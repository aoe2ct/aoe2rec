use binrw::io::{BufReader, Cursor};
use binrw::{binrw, BinReaderExt, BinResult, BinWriterExt, NullString};
use clap::Parser;
use std::fs::File;

#[derive(Parser)]
struct Args {
    // Path to the aoe2record file
    #[arg(short, long)]
    file: std::path::PathBuf,
}
fn main() {
    let args = Args::parse();
    if !args.file.is_file() {
        println!("File not found");
        std::process::exit(-1);
    }
    println!("Parsing {}", args.file.to_str().unwrap_or("<missing>"));
    read_file(&args.file);
}

#[binrw]
pub struct EncodedHeader {
    length: u32,
    other: u32,
    #[br(count = length - 8)]
    zheader: Vec<u8>,
    log_version: u32,
}

#[derive(Debug)]
#[binrw]
pub struct RecHeader {
    game: NullString,
    save: f32,
    version_minor: u16,
    version_major: u16,
    build: u32,
    timestamp: i32,
    version_2: [u16; 2],
    interval_version: [u16; 2],
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
    unknown2: u32,
    unknown3: u32,
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings1: Vec<DeString>,
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings2: Vec<DeString>,
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings3: Vec<DeString>,
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings4: Vec<DeString>,
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings5: Vec<DeString>,
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings6: Vec<DeString>,
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings7: Vec<DeString>,
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings8: Vec<DeString>,
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings9: Vec<DeString>,
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings10: Vec<DeString>,
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings11: Vec<DeString>,
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings12: Vec<DeString>,
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings13: Vec<DeString>,
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings14: Vec<DeString>,
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings15: Vec<DeString>,
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings16: Vec<DeString>,
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings17: Vec<DeString>,
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings18: Vec<DeString>,
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings19: Vec<DeString>,
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    other_strings20: Vec<DeString>,
    num_strategic_numbers: u32,
    #[br(count = num_strategic_numbers)]
    strategic_numbers: Vec<i32>,
    num_ai_files: u32,
    #[br(count = num_ai_files)]
    ai_files: Vec<AIFile>,
    unknown4: u32, // 25.02
    unknown5: u32, // 25.02
    unknown6: u32, // 25.02
    guid: [u8; 16],
    lobby_name: DeString,
    unknown7: [u8; 8], // 25.22
    modded_dataset: DeString,
    unknown8: [u8; 19],
    unknown9: [u8; 5],   // 13.13
    unknown10: [u8; 3],  // 13.17
    unknown11: DeString, // 13.17
    unknown12: [u8; 3],  // 13.17
    unknown13: u8,       // 20.06
    unknown14: [u8; 8],  // 20.16
    unknown15: [u8; 21], // 25.06
    unknown16: [u8; 4],  // 25.22
    unknown17: [u8; 8],  // 26.16
    unknown18: [u8; 3],  // 37
    unknown19: [u8; 8],  // 50
    #[br(if(version_major >= 63))]
    unknown24: Option<[u8; 5]>,
    unknown20: DeString,
    unknown21: [u8; 5],
    unknown22: u8,      // 13.13
    unknown23: [u8; 2], // 13.13
    ver37: [u32; 2],
}

#[derive(Debug)]
#[binrw]
pub struct Player {
    dlc_id: u32,
    color_id: i32,
    selected_color: u8,
    selected_team_id: u8,
    resolved_team_id: u8,
    dat_crc: [u8; 8],
    mp_game_version: u8,
    civ_id: u32,
    unknown: u32,
    ai_type: DeString,
    ai_civ_name_index: u8,
    ai_name: DeString,
    name: DeString,
    player_type: u32,
    profile_id: u32,
    // #[br(magic = b"\x00\x00\x00\x00")]
    ai: [u8; 4],
    player_number: i32,
    prefer_random: u8,
    custom_ai: u8,
    handicap: [u8; 8],
}

#[derive(Debug)]
#[binrw]
struct EmptySlot {
    i0x: u32,
    i0a: u32,
    unknown1: u32,
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

#[derive(Debug)]
#[binrw]
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

#[binrw]
pub struct DeString {
    #[br(magic = b"\x60\x0A")]
    length: u16,
    #[br(count = length)]
    value: Vec<u8>,
}

impl std::fmt::Debug for DeString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::string::String::from_utf8_lossy(&self.value))
    }
}

fn read_file(path: &std::path::Path) -> std::io::Result<()> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let encoded_header: EncodedHeader = reader.read_le().unwrap();
    println!(
        "Header length: {} {}",
        encoded_header.length, encoded_header.other
    );
    let (header, _) = yazi::decompress(&encoded_header.zheader, yazi::Format::Raw).unwrap();
    println!(
        "First 8 bytes: {}, {}, {}, {}, {}, {}, {}, {}",
        header[0], header[1], header[2], header[3], header[4], header[5], header[6], header[7]
    );
    println!("Log version: {}", encoded_header.log_version);
    let mut hreader = Cursor::new(header);
    let parsed_header: RecHeader = hreader.read_le().unwrap();
    println!("{:?}", parsed_header);

    Ok(())
}

#[binrw::parser(reader, endian)]
fn read_strings_of_length() -> BinResult<Vec<DeString>> {
    let mut strings: Vec<DeString> = Vec::new();
    loop {
        let crc: u32 = reader.read_type(endian)?;
        println!("crc: {}", crc);
        if crc > 0 && crc < 255 {
            break;
        }
        let string: DeString = reader.read_type(endian)?;
        strings.push(string);
    }
    Ok(strings)
}

#[binrw::writer(writer, endian)]
fn write_len_and_string(strings: &Vec<DeString>) -> BinResult<()> {
    // for string in strings {
    //     writer.write_type(&u32::try_from(string.len()).unwrap(), endian)?;
    //     writer.write_type(&string, endian)?;
    // }
    // writer.write_type(&0u32, endian)?;
    Ok(())
}
