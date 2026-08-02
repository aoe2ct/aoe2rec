pub mod actions;
pub mod header;
pub mod minimal;
pub mod summary;

use binrw::helpers::until_eof;
use binrw::io::{BufReader, Cursor, SeekFrom, TakeSeekExt};
use binrw::{binrw, BinReaderExt, BinResult, BinWriterExt, NullString};
use header::{decompress, RecHeader};
use serde::Serialize;
use std::error::Error;
use std::fs::File;
use summary::GameTeam;

#[binrw]
#[derive(Serialize)]
#[br(stream = s)]
pub struct Savegame {
    #[br(parse_with=until_eof)]
    pub chapters: Vec<Chapter>,
}

fn chapter_size(current_offset: u64, next_offset: u64) -> u64 {
    if next_offset == 0 {
        return u32::MAX as u64;
    }
    next_offset.saturating_sub(current_offset)
}

#[binrw]
#[derive(Serialize)]
#[br(stream = s)]
pub struct Chapter {
    #[br(dbg)]
    pub header_end: u32,
    #[br(calc=<u64>::from(header_end) - s.stream_position().unwrap())]
    header_len: u64,
    #[br(dbg)]
    pub next_chapter_address: u32,
    // #[br(if(next_chapter_address >0), parse_with=binrw::FilePtr32::parse, ski)]
    // pub next_chapter: Option<Box<Savegame>>,
    #[br(count = header_len - 4, map = decompress)]
    pub zheader: RecHeader,
    #[br(temp, try_calc= s.stream_position())]
    #[bw(ignore)]
    current_address: u64,
    #[br(map_stream = |r| r.take_seek(chapter_size(current_address, next_chapter_address as u64)))]
    #[br(parse_with = until_eof, args(zheader.version_major))]
    pub operations: Vec<Operation>,
}

#[binrw]
#[br(stream = s)]
#[derive(Serialize, Debug)]
pub struct ChapterData {
    chapter_end: u32,
    chapter_address: u32,
    #[br(calc=s.stream_position().unwrap())]
    current_position: u64,
    #[br(count = (chapter_end as u64) - current_position)]
    chapter_data: Vec<u8>,
}

#[binrw]
#[derive(Serialize, Debug)]
#[br(import(major: u16))]
pub enum Operation {
    #[br(magic = 1u32)]
    Action {
        length: u32,
        #[br(pad_size_to = length, args(length, major))]
        action_data: actions::ActionData,
        world_time: u32,
    },
    #[br(magic = 2u32)]
    Sync {
        time_increment: u32,
        #[br(restore_position)]
        next: u32,
        #[br(if (next == 0))]
        checksum: Option<SyncChecksum>,
    },
    #[br(magic = 3u32)]
    Viewlock { x: f32, y: f32, player_id: u32 },
    #[br(magic = 4u32)]
    Chat { padding: [u8; 4], text: LenString },
    #[br(magic = 5u32)]
    Pregame {
        checksum_interval: u32,
        multiplayer: Bool32,
        rec_owner: u32,
        reveal_map: Bool32,
        use_sequence_numbers: Bool32,
        number_of_chapters: u32,
        aok_or_de: Bool32,
    },
    #[br(magic = 6u32)]
    PostGame {
        #[br(seek_before = SeekFrom::End(-12))]
        version: u32,
        #[br(seek_before = SeekFrom::Current(-8))]
        num_blocks: u32,
        #[br(count = num_blocks, seek_before = SeekFrom::Current(-8), restore_position)]
        blocks: Vec<PostGameBlock>,
        version_repeat: u32,
        #[br(magic = b"\xce\xa4\x59\xb1\x05\xdb\x7b\x43")]
        end_bit: (),
    },
}

#[binrw]
#[derive(Serialize, Debug)]
pub enum PostGameBlock {
    #[br(magic = 1u32)]
    WorldTime {
        #[br(seek_before=SeekFrom::Current(-8))]
        length: u32,
        #[br(seek_before=SeekFrom::Current(-(length as i64) - 4))]
        world_time: u32,
    },
    #[br(magic = 2u32)]
    Leaderboards {
        #[br(seek_before = SeekFrom::Current(-8))]
        length: u32,
        #[br(seek_before = SeekFrom::Current(-(length as i64) - 4))]
        num_leaderboards: u32,
        #[br(count = num_leaderboards)]
        leaderboards: Vec<Leaderboard>,
        #[br(seek_before = SeekFrom::Current(-(length as i64) - 4))]
        realignment_field: (),
    },
}

#[binrw]
#[derive(Serialize, Debug)]
pub struct Leaderboard {
    pub id: u32,
    pub unknown1: u16,
    pub num_players: u32,
    #[br(count = num_players)]
    pub players: Vec<LeaderboardPlayer>,
}

#[binrw]
#[derive(Serialize, Debug)]
pub struct LeaderboardPlayer {
    pub player_number: i32,
    pub rank: i32,
    pub elo: i32,
}

#[binrw]
#[derive(Serialize, Debug)]
pub struct SyncChecksum {
    pub unknown1: [u8; 8],
    pub sync: u32,
    pub unknown2: [u8; 4],
    pub sequence: u32,
    #[serde(skip_serializing)]
    #[br(if (sequence > 0))]
    pub unknown3: Option<[u8; 332]>,
    pub unknown4: [u8; 8],
}

#[binrw]
#[derive(Serialize)]
#[br(repr = u32)]
#[bw(repr = u32)]
pub enum OperationType {
    Action = 1,
    Sync,
    Viewlock,
    Chat,
}

#[binrw]
#[derive(Serialize)]
pub struct SyncOperation {}

#[binrw]
#[derive(Copy, Clone)]
pub struct Bool {
    #[br(map = |x: u8| x == 1)]
    #[bw(map = |ranked: &bool| match ranked { true => 1u8, false => 0u8})]
    value: bool,
}

#[binrw]
#[derive(Copy, Clone)]
pub struct Bool32 {
    #[br(map = |x: u32| x == 1)]
    #[bw(map = |x: &bool| if *x { 1u32 } else {  0u32})]
    value: bool,
}

impl Serialize for Bool32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bool(self.value)
    }
}

impl From<Bool32> for bool {
    fn from(val: Bool32) -> Self {
        val.value
    }
}

impl std::fmt::Debug for Bool32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
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

impl From<bool> for Bool {
    fn from(value: bool) -> Self {
        Bool { value }
    }
}

impl From<Bool> for bool {
    fn from(val: Bool) -> Self {
        val.value
    }
}

#[binrw]
#[derive(Debug)]
pub struct LenString {
    length: u32,
    #[br(count = length)]
    value: Vec<u8>,
}

// TODO: Implement this with a generic?
#[binrw]
pub struct LenString16 {
    length: u16,
    #[br(count = length)]
    value: Vec<u8>,
}

#[binrw]
#[derive(Clone, Default)]
pub struct DeString {
    #[brw(magic = b"\x60\x0A")]
    #[bw(calc(value.len().try_into().unwrap()))]
    length: u16,
    #[br(count = length)]
    value: Vec<u8>,
}

impl From<&DeString> for String {
    fn from(value: &DeString) -> Self {
        std::string::String::from_utf8_lossy(&value.value).to_string()
    }
}

impl From<&String> for DeString {
    fn from(value: &String) -> Self {
        Self {
            value: value.as_bytes().to_vec(),
        }
    }
}

impl From<DeString> for String {
    fn from(val: DeString) -> Self {
        std::string::String::from_utf8_lossy(&val.value).to_string()
    }
}

#[binrw]
#[derive(Debug, Clone)]
pub struct MyNullString {
    text: NullString,
}

impl From<String> for MyNullString {
    fn from(value: String) -> Self {
        MyNullString { text: value.into() }
    }
}

impl From<MyNullString> for String {
    fn from(val: MyNullString) -> Self {
        val.text.to_string()
    }
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

impl Serialize for LenString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let strvalue = std::string::String::from_utf8_lossy(&self.value);
        serializer.serialize_str(&strvalue)
    }
}

impl Serialize for LenString16 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let strvalue = std::string::String::from_utf8_lossy(&self.value);
        serializer.serialize_str(&strvalue)
    }
}

impl std::fmt::Debug for LenString16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::string::String::from_utf8_lossy(&self.value))
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

impl Savegame {
    pub fn from_bytes(data: bytes::Bytes) -> Result<Savegame, Box<dyn Error>> {
        let mut breader = BufReader::new(Cursor::new(data));
        let savegame: Savegame = breader.read_le()?;
        Ok(savegame)
    }
    pub fn from_file(path: &std::path::Path) -> Result<Savegame, Box<dyn Error>> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let savegame: Savegame = reader.read_le()?;
        Ok(savegame)
    }

    pub fn get_duration(&self) -> u32 {
        self.chapters[0].operations.iter().fold(
            self.chapters[0].zheader.replay.world_time,
            |duration, operation| match operation {
                Operation::Sync { time_increment, .. } => duration + time_increment,
                _ => duration,
            },
        )
    }

    pub fn get_resignations(&self) -> Vec<u8> {
        self.chapters
            .iter()
            .flat_map(|chapter| {
                chapter.operations.iter().map(|operation| match operation {
                    Operation::Action {
                        action_data: actions::ActionData::Resign { player_id, .. },
                        ..
                    } => *player_id,
                    _ => 100,
                })
            })
            .filter(|player_id| *player_id < 100)
            .collect()
    }

    pub fn get_summary(&self) -> summary::SavegameSummary<'_> {
        summary::SavegameSummary {
            header: summary::SummaryHeader {
                game: &self.chapters[0].zheader.game,
                version_minor: self.chapters[0].zheader.version_minor,
                version_major: self.chapters[0].zheader.version_major,
                build: self.chapters[0].zheader.build,
                timestamp: self.chapters[0].zheader.timestamp,
                game_settings: &self.chapters[0].zheader.game_settings,
                replay: &self.chapters[0].zheader.replay,
            },
            duration: self.get_duration(),
            resignations: self.get_resignations(),
            teams: GameTeam::from_savegame(self),
        }
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

#[binrw::writer(writer, endian)]
fn write_len_and_string(strings: &Vec<DeString>) -> BinResult<()> {
    for string in strings {
        writer.write_type(&string, endian)?;
    }
    writer.write_type(&0u32, endian)?;
    Ok(())
}
