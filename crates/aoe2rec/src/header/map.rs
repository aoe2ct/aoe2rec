use crate::Bool;
use binrw::binrw;
use serde::Serialize;

#[binrw]
#[derive(Serialize)]
pub struct MapInfo {
    pub size_x: u32,
    pub size_y: u32,
    pub zone_count: u32,
    #[br(args { inner: (size_x * size_y,)} )]
    #[br(count = zone_count)]
    pub ignored_map_tiles: Vec<IgnoreMapTile>,
    pub all_visible: Bool,
    pub fog_of_war: Bool,
    #[br(count = size_x * size_y)]
    pub tiles: Vec<Tile>,
    pub num_data: u32,
    pub unknown1: [u8; 4],
    #[br(count = num_data * 4)]
    pub unknown2: Vec<u8>,
    #[br(count = num_data)]
    pub unknown_data: Vec<UnknownData>,
    #[br(assert(size_x == size_x_2))]
    pub size_x_2: u32,
    #[br(assert(size_y == size_y_2))]
    pub size_y_2: u32,
    #[br(count = size_x * size_y * 4)]
    pub visibility: Vec<u8>,
    #[br(count = size_x * size_y * 4)]
    pub unknown3: Vec<u8>,
}

#[binrw]
#[derive(Serialize)]
#[br(import(tile_count: u32))]
pub struct IgnoreMapTile {
    pub tile_num: u32,
    #[serde(skip_serializing)]
    pub unknown1: [u8; 2044],
    #[serde(skip_serializing)]
    #[br(count = tile_count)]
    pub unknown_tiles: Vec<u16>,
    pub float_count: u32,
    #[serde(skip_serializing)]
    #[br(count = float_count)]
    pub unknown2: Vec<f32>,
    #[serde(skip_serializing)]
    pub unknown3: u32,
}

#[binrw]
#[derive(Serialize, Debug)]
pub struct Tile {
    pub terrain_type: u8,
    pub unknown1: u8,
    pub terrain_type_2: u8,
    pub elevation: u8,
    pub unknown2: u16,
    pub unknown3: u16,
    pub unknown4: u16,
}

#[binrw]
#[derive(Serialize, Debug)]
pub struct UnknownData {
    pub num_obstructions: u32,
    #[br(count = num_obstructions * 8)]
    pub obstructions: Vec<u8>,
}
