mod actions;
mod header;
mod minimal;
mod primitives;
pub mod summary;
mod tests;

use binrw::helpers::until_eof;
use binrw::io::{BufReader, Cursor};
use binrw::{binread, binrw, BinRead, BinReaderExt, BinResult, BinWriterExt};
use primitives::{DeString, LenString32, Bool};
use header::{decompress, ChapterHeader};
use serde::Serialize;
use std::error::Error;
use std::fs::File;
use summary::GameTeam;

//#[binrw] // TODO: make writable again
#[binread]
#[derive(Serialize)]
pub struct Savegame {
    #[br(parse_with = until_eof)]
    chapters: Vec<Chapter>,
}

// TODO: Can we simplify this?
fn until_chapter_end<'a, Ret, T, Arg, Reader>(
    next_header_addr: u32,
) -> impl Fn(&mut Reader, binrw::Endian, Arg) -> BinResult<Ret>
where
    Ret: FromIterator<T>,
    Arg: Clone,
    T: BinRead<Args<'a> = Arg>,
    Reader: binrw::io::Read + binrw::io::Seek,
{
    move |reader, endian, args| {
        std::iter::from_fn(|| {
            if next_header_addr == 0 {
                match T::read_options(reader, endian, args.clone()) {
                    ok @ Ok(_) => Some(ok),
                    Err(err) if err.is_eof() => None,
                    err => Some(err),
                }
            } else {
                let chapter_end = reader
                    .stream_position()
                    .map_or(true, |pos| pos >= next_header_addr.into());
                if chapter_end {
                    None
                } else {
                    match T::read_options(reader, endian, args.clone()) {
                        Ok(ok) => Some(Ok(ok)),
                        Err(err) => Some(Err(err)),
                    }
                }
            }
        })
        .fuse()
        .collect()
    }
}

#[binread]
#[brw(little, stream = stream)] // TODO: re-introduce write-capability
#[derive(Serialize)]
pub struct Chapter {
    #[br(temp)]
    // #[bw(try_calc = )] // TODO: calculate address
    header_end_address: u32,
    #[br(temp)]
    // #[bw(try_calc = )] // TODO: calculate address
    next_header_addr: u32,

    #[br(try_calc = (Into::<u64>::into(header_end_address) - stream.stream_position()?).try_into())]
    header_length: u32,

    #[br(count = header_length, map = decompress)]
    //#[bw(map = compress)] // TODO: compression
    header: ChapterHeader,
    log_version: u32,
    meta: ChapterMeta,

    #[br(parse_with = until_chapter_end(next_header_addr))]
    operations: Vec<Operation>,
}

#[binrw]
#[derive(Serialize, Debug)]
pub struct ChapterMeta {
    pub checksum_interval: u32,
    #[brw(pad_after = 3)]
    pub multiplayer: Bool,
    pub rec_owner: u32,
    #[brw(pad_after = 3)]
    pub reveal_map: Bool,
    pub use_sequence_numbers: u32,
    pub number_of_chapters: u32,
    pub aok_or_de: u32,
}

#[binrw]
#[derive(Serialize, Debug)]
#[brw(import(major: u16))]
pub enum Operation {
    #[brw(magic = b"\xCE\xA4\x59\xB1\x05\xDB\x7B\x43")]
    EndOfReplay,

    #[brw(magic = 1u32)]
    Action {
        length: u32,
        #[br(pad_size_to = length, args(major))]
        action_data: actions::ActionData,
        world_time: u32,
    },
    #[brw(magic = 2u32)]
    Sync {
        time_increment: u32,
    },
    #[brw(magic = 3u32)]
    Viewlock { x: f32, y: f32, player_id: u32 },
    #[brw(magic = 4u32)]
    Chat { padding: [u8; 4], text: LenString32 },
    #[brw(magic = 5u32)]
    AddAttribute {
        player_id: u8,
        #[br(pad_after = 1)]
        attribute: u8,
        amount: f32,
    },
    #[brw(magic = 6u32)]
    PostGame {
        game_length: u32,
        unk1: u32,
        unk2: u32,

        #[br(temp)]
        #[bw(try_calc = leaderboards.len().try_into())]
        num_leaderboards: u32,

        #[br(temp)]
        #[bw(try_calc = leaderboards.len().try_into())]
        num_leaderboards: u32,
        #[br(count = num_leaderboards)]
        leaderboards: Vec<PostGameLeaderboard>,

        unk3: u32,
        unk4: u32,
        unk5: u32,
        unk6: u32,
    },
}

#[binrw]
#[derive(Serialize, Debug)]
pub struct PostGameLeaderboard {
    pub id: u32,
    pub unk1: u8,
    pub unk2: u8,

    #[br(temp)]
    #[bw(try_calc = players.len().try_into())]
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
#[brw(repr = u32)]
pub enum OperationType {
    Action = 1,
    Sync,
    Viewlock,
    Chat,
}

#[binrw]
#[derive(Serialize)]
pub struct SyncOperation {}

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

    pub fn operations(&self) -> impl Iterator<Item = &Operation> {
        self.chapters.iter().flat_map(|chapter|chapter.operations.iter())
    }

    pub fn get_duration(&self) -> u32 {
        self.operations().fold(
            self.chapters[0].header.replay.world_time,
            |duration, operation| {
                
                match operation {
                    Operation::Sync { time_increment, .. } => duration + time_increment,
                    _ => duration,
                }
            },
        )
    }

    pub fn get_resignations(&self) -> Vec<u8> {
        self.operations()
            .map(|operation| match operation {
                Operation::Action { action_data: actions::ActionData::Resign { player_id, .. }, .. } => *player_id,
                _ => 100,
            })
            .filter(|player_id| *player_id < 100)
            .collect()
    }

    pub fn get_summary(&self) -> summary::SavegameSummary<'_> {
        summary::SavegameSummary {
            header: summary::SummaryHeader {
                game: &self.chapters[0].header.game,
                version_minor: self.chapters[0].header.version_minor,
                version_major: self.chapters[0].header.version_major,
                build: self.chapters[0].header.build,
                timestamp: self.chapters[0].header.timestamp,
                game_settings: &self.chapters[0].header.game_settings,
                replay: &self.chapters[0].header.replay,
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
