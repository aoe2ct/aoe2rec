use binrw::{binrw, BinReaderExt, BinResult};
use serde::Serialize;

use crate::DeString;

#[binrw]
#[derive(Serialize, Debug)]
pub struct AIInfo {
    #[br(parse_with = skip_ai)]
    skip: (), // #[br(dbg)]
              // max_strings: u16,
              // #[br(dbg)]
              // strings_count: u16,
              // #[br(dbg)]
              // unknown1: u32,
              // #[br(count=strings_count)]
              // ai_strings: Vec<LenString>,
              // unknown2: [u8; 3],
              // unknown3: u16,
              // unknown4: u16,
              // unknown5: [u32; 2],
              // #[br(dbg)]
              // unknown_max: u16,
              // #[br(dbg)]
              // unknown_count: u16,
              // #[br(dbg)]
              // unknown6: u32,
              // #[br(count = unknown_count, dbg)]
              // unknown7: Vec<UnknownAI>,
              // unknown_max2: u16,
              // unknown_count2: u16,
              // unknown8: [u32; 3],
              // #[br(count = unknown_count2)]
              // unknown9: Vec<UnknownAI>,
              // unknown_max3: u16,
              // unknown_count3: u16,
              // unknown10: [u32; 3],
              // #[br(count = unknown_count3, dbg)]
              // unknown11: Vec<UnknownAI>,
              // #[br(magic = b"\x00", count = 4096)]
              // skip_magic3: Vec<()>,
}

#[binrw]
#[derive(Serialize, Debug)]
pub struct UnknownAI {
    #[br(dbg)]
    seq: u16,
    // #[br(magic = b"\xff\xff")]
    #[br(dbg)]
    skip_magic: u16,
    #[br(dbg)]
    unknown1: u8,
    #[br(dbg)]
    unknown_count: u8,
    #[br(magic = b"\x00\x00", dbg)]
    skip_magic2: u16,
    #[br(count = unknown_count)]
    unknown2: Vec<u32>,
    #[br(dbg)]
    unknown3: u32,
    #[br(dbg)]
    unknown4: u32,
    #[br(dbg)]
    unknown5: u32,
    #[br(dbg)]
    crc: u32,
}

#[binrw]
#[derive(Serialize, Debug)]
pub struct AIFile {
    pub unknown: u32,
    pub name: DeString,
    pub unknown2: u32,
}

#[binrw::parser(reader, endian)]
fn skip_ai() -> BinResult<()> {
    let mut null_count = 0;
    loop {
        let next_byte: u8 = reader.read_type(endian)?;
        if next_byte == 0 {
            null_count += 1
        } else {
            null_count = 0;
        }
        if null_count == 4096 {
            break;
        }
    }
    Ok(())
}
