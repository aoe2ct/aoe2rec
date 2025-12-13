use std::{error::Error, fs::File, io::Cursor};

use binrw::{binrw, helpers::until_eof, io::BufReader, BinReaderExt, BinWrite};

use crate::{
    header::{EmptySlot, Player},
    primitives::{DeString, MyNullString},
    read_strings_of_length, write_len_and_string,
};

pub fn decompress(header_data: Vec<u8>) -> MinimalHeader {
    let (header, _) = yazi::decompress(&header_data, yazi::Format::Raw).unwrap();
    let mut hreader = BufReader::new(Cursor::new(header));
    let parsed_header: MinimalHeader = hreader.read_le().unwrap();

    parsed_header
}

pub fn compress(header_data: &MinimalHeader) -> Vec<u8> {
    let mut output = Cursor::new(vec![]);
    header_data.write(&mut output).unwrap();
    yazi::compress(
        output.get_ref(),
        yazi::Format::Raw,
        yazi::CompressionLevel::Default,
    )
    .unwrap()
}

#[binrw]
#[brw(little)]
pub struct MinimalSave {
    #[bw(calc = 8u32 + u32::try_from(compress(zheader).len()).unwrap())]
    pub length: u32,
    pub other: u32,
    #[br(count = length - 8, map = decompress)]
    #[bw(map = compress)]
    pub zheader: MinimalHeader,
    #[br(parse_with = until_eof)]
    pub rest: Vec<u8>,
}

#[binrw]
#[brw(little)]
pub struct MinimalHeader {
    pub game: MyNullString,
    pub save: f32,
    #[br(map = |x: u16| (x as f32 / 6553.6).round() as u16)]
    pub version_minor: u16,
    pub version_major: u16,
    pub build: u32,
    pub rest1: [u8; 12],
    #[br(args(version_major))]
    pub game_settings: MinimalGameSettings,
    #[br(parse_with = until_eof)]
    pub rest: Vec<u8>,
}
#[binrw]
#[brw(little, import(major: u16))]
pub struct MinimalGameSettings {
    pub game_options_version: u32,
    #[bw(calc(dlcs.len().try_into().unwrap()))]
    pub n_dlc: u32,
    #[br(count=n_dlc)]
    pub dlcs: Vec<u32>,
    ignore1: [u32; 15],
    #[bw(calc(players.len().try_into().unwrap()))]
    pub n_players: u32,
    ignore2: [u32; 2],
    ignore3: [u8; 34],
    #[br(count = n_players, args { inner: (major,)})]
    pub players: Vec<Player>,
    ignore4: [u8; 12],
    #[br(count = 8 - n_players, args { inner: (major,)})]
    pub empty_slots: Vec<EmptySlot>,
    ignore5: [u8; 17],
    #[br(parse_with = read_strings_of_length)]
    #[bw(write_with = write_len_and_string)]
    pub rms_strings: Vec<DeString>,
    // #[serde(skip_serializing)]
    // pub unknown2: u32,
    // #[serde(skip_serializing)]
    // pub unknown3: u32,
    // #[serde(skip_serializing)]
    // #[br(parse_with = read_strings_of_length)]
    // #[bw(write_with = write_len_and_string)]
    // pub other_strings: Vec<DeString>,
    // #[serde(skip_serializing)]
    // #[br(parse_with = read_strings_of_length)]
    // #[bw(write_with = write_len_and_string)]
    // pub other_strings2: Vec<DeString>,
    // #[serde(skip_serializing)]
    // #[br(parse_with = read_strings_of_length)]
    // #[bw(write_with = write_len_and_string)]
    // pub other_strings3: Vec<DeString>,
    // #[serde(skip_serializing)]
    // #[br(parse_with = read_strings_of_length)]
    // #[bw(write_with = write_len_and_string)]
    // pub other_strings4: Vec<DeString>,
    // #[serde(skip_serializing)]
    // #[br(parse_with = read_strings_of_length)]
    // #[bw(write_with = write_len_and_string)]
    // pub other_strings5: Vec<DeString>,
    // #[serde(skip_serializing)]
    // #[br(parse_with = read_strings_of_length)]
    // #[bw(write_with = write_len_and_string)]
    // pub other_strings6: Vec<DeString>,
    // #[serde(skip_serializing)]
    // #[br(parse_with = read_strings_of_length)]
    // #[bw(write_with = write_len_and_string)]
    // pub other_strings7: Vec<DeString>,
    // #[serde(skip_serializing)]
    // #[br(parse_with = read_strings_of_length)]
    // #[bw(write_with = write_len_and_string)]
    // pub other_strings8: Vec<DeString>,
    // #[serde(skip_serializing)]
    // #[br(parse_with = read_strings_of_length)]
    // #[bw(write_with = write_len_and_string)]
    // pub other_strings9: Vec<DeString>,
    // #[serde(skip_serializing)]
    // #[br(parse_with = read_strings_of_length)]
    // #[bw(write_with = write_len_and_string)]
    // pub other_strings10: Vec<DeString>,
    // #[serde(skip_serializing)]
    // #[br(parse_with = read_strings_of_length)]
    // #[bw(write_with = write_len_and_string)]
    // pub other_strings11: Vec<DeString>,
    // #[serde(skip_serializing)]
    // #[br(parse_with = read_strings_of_length)]
    // #[bw(write_with = write_len_and_string)]
    // pub other_strings12: Vec<DeString>,
    // #[serde(skip_serializing)]
    // #[br(parse_with = read_strings_of_length)]
    // #[bw(write_with = write_len_and_string)]
    // pub other_strings13: Vec<DeString>,
    // #[serde(skip_serializing)]
    // #[br(parse_with = read_strings_of_length)]
    // #[bw(write_with = write_len_and_string)]
    // pub other_strings14: Vec<DeString>,
    // #[serde(skip_serializing)]
    // #[br(parse_with = read_strings_of_length)]
    // #[bw(write_with = write_len_and_string)]
    // pub other_strings15: Vec<DeString>,
    // #[serde(skip_serializing)]
    // #[br(parse_with = read_strings_of_length)]
    // #[bw(write_with = write_len_and_string)]
    // pub other_strings16: Vec<DeString>,
    // #[serde(skip_serializing)]
    // #[br(parse_with = read_strings_of_length)]
    // #[bw(write_with = write_len_and_string)]
    // pub other_strings17: Vec<DeString>,
    // #[serde(skip_serializing)]
    // #[br(parse_with = read_strings_of_length)]
    // #[bw(write_with = write_len_and_string)]
    // pub other_strings18: Vec<DeString>,
    // #[serde(skip_serializing)]
    // #[br(parse_with = read_strings_of_length)]
    // #[bw(write_with = write_len_and_string)]
    // pub other_strings19: Vec<DeString>,
    // #[serde(skip_serializing)]
    // #[br(parse_with = read_strings_of_length)]
    // #[bw(write_with = write_len_and_string)]
    // pub other_strings20: Vec<DeString>,
    // #[serde(skip_serializing)]
    // pub num_strategic_numbers: u32,
    // #[br(count = num_strategic_numbers)]
    // pub strategic_numbers: Vec<i32>,
    // pub num_ai_files: u32,
    // #[br(count = num_ai_files)]
    // pub ai_files: Vec<AIFile>,
    // #[serde(skip_serializing)]
    // pub unknown4: u32, // 25.02
    // #[serde(skip_serializing)]
    // pub unknown5: u32, // 25.02
    // #[serde(skip_serializing)]
    // pub unknown6: u32, // 25.02
    // pub guid: [u8; 16],
    // pub lobby_name: DeString,
    // #[serde(skip_serializing)]
    // pub unknown7: [u8; 8], // 25.22
    // pub modded_dataset: DeString,
    // #[serde(skip_serializing)]
    // pub unknown8: [u8; 19],
    // #[serde(skip_serializing)]
    // pub unknown9: [u8; 5], // 13.13
    // #[serde(skip_serializing)]
    // pub unknown10: [u8; 3], // 13.17
    // #[serde(skip_serializing)]
    // pub unknown11: DeString, // 13.17
    // #[serde(skip_serializing)]
    // pub unknown12: [u8; 3], // 13.17
    // #[serde(skip_serializing)]
    // pub unknown13: u8, // 20.06
    // #[serde(skip_serializing)]
    // pub unknown14: [u8; 8], // 20.16
    // #[serde(skip_serializing)]
    // pub unknown15: [u8; 21], // 25.06
    // #[serde(skip_serializing)]
    // pub unknown16: [u8; 4], // 25.22
    // #[serde(skip_serializing)]
    // pub unknown17: [u8; 8], // 26.16
    // #[serde(skip_serializing)]
    // pub unknown18: [u8; 3], // 37
    // #[serde(skip_serializing)]
    // pub unknown19: [u8; 8], // 50
    // #[serde(skip_serializing)]
    // #[br(if(major >= 63))]
    // pub unknown24: Option<[u8; 5]>,
    // #[serde(skip_serializing)]
    // pub unknown20: DeString,
    // #[serde(skip_serializing)]
    // pub unknown21: [u8; 5],
    // #[serde(skip_serializing)]
    // pub unknown22: u8, // 13.13
    // #[serde(skip_serializing)]
    // pub unknown23: [u8; 2], // 13.13
    // pub timestamp: u32,
    // #[serde(skip_serializing)]
    // pub unknown25: u32,
}

impl MinimalSave {
    pub fn from_file(path: &std::path::Path) -> Result<MinimalSave, Box<dyn Error>> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let savegame: MinimalSave = reader.read_le()?;
        Ok(savegame)
    }

    pub fn from_bytes(data: bytes::Bytes) -> Result<MinimalSave, Box<dyn Error>> {
        let mut breader = BufReader::new(Cursor::new(data));
        let savegame: MinimalSave = breader.read_le()?;
        Ok(savegame)
    }
}
