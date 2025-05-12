#pragma endian little
#pragma pattern_limit 10000000
#pragma array_limit 1000000
import std.mem;
import std.file;
import hex.dec;
import hex.provider;
import type.magic;


struct Action {
  u32 length;
  u8 action_data[length + 4];
};

struct Sync {
  u32 time_increment;
  if(std::mem::read_unsigned($, 4) == 0) {
    padding[8];
    u32 sync;
    u32 unknown1;
    u32 sequence;
    if (sequence > 0) {
        u8 unknown2[332];
    }
    u8 unknown3[8];
  }
};

struct Viewlock {
  float x;
  float y;
  u32 player_id;
};

struct Chat {
  padding[4];
  std::string::SizedString<u32> text;
};

enum PostgameBlockType: u32 {
    WorldTime = 1,
    Leaderboards = 2
};

struct WorldTime {
    u32 world_time;
};

//struct PostgameBlock {
//    PostgameBlockType identifier;
//    match (identifier) {
//    (PostgameBlockType::WorldTime): WorldTime [[inline]];
//            (_): std::error(std::format("Unknown postgame block type: {} at 0x{:X}", u32(identifier), $));
//    }
//};

struct PostgameBlock {
    u32 identifier;
    u32 length;
    u32 work_time;
};

struct Postgame {
  u32 version @ std::mem::size() - 12;
  u32 num_blocks @ std::mem::size() - 16;
  PostgameBlock blocks[num_blocks];
};

enum OperationType: u32 {
    Action = 1,
    Sync = 2,
    Viewlock = 3,
    Chat = 4,
    Postgame = 6,
};


struct Operation {
    OperationType type;
    match (type) {
        (OperationType::Sync): Sync [[inline]];
        (OperationType::Viewlock): Viewlock [[inline]];
        (OperationType::Chat): Chat [[inline]];
        (OperationType::Action): Action [[inline]];
        // (OperationType::Postgame): Postgame [[inline]];
        (OperationType::Postgame): break;
        (_): std::error(std::format("Unknown action type: {} at 0x{:X}", u32(type), $));
    }
};

struct Meta {
    u32 checksum_interval;
    bool multiplayer;
    padding[3];
    u32 rec_owner;
    bool reveal_map;
    padding[3];
    u32 use_sequence_numbers;
    u32 number_of_chapters;
    u32 aok_or_de;
};

struct Aoe2Save {
    u32 hlen;
    u32 check;
    u8 compressed[hlen - 8];
    u32 log_version;
    Meta meta;
    //Operation operations1[245503];
    Operation operations[while($ < std::mem::size())];
};


Aoe2Save savegame @ 0x00;

std::mem::Section header = std::mem::create_section("Header");
hex::dec::zlib_decompress(savegame.compressed, header, -15);

// To further analyze the rec, click on "Sections" below and
// then click on the rightmost button "Export content" and
// save it to a different file.
// You can then use the decompressed.hexpat pattern to analyze the rec.