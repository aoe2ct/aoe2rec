#pragma endian little
#pragma pattern_limit 10000000
#pragma array_limit 1000000
import std.mem;
// import std.file;
import hex.dec;
import hex.provider;
import hex.core;
// import type.magic;
float game_version;
import aoe2operations;

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

struct HeaderStart {
    u8 rec_version[7];
    type::Magic<"\x00">;
    float checker;
    u16 version_minor;
    u16 version_major;
};

struct Aoe2Save {
    u32 hlen;
    u32 check;
    u8 compressed[hlen - 8];
    std::mem::Section header = std::mem::create_section("Header");
    hex::dec::zlib_decompress(compressed, header, -15);
    HeaderStart header_start @ 0x0 in header;
    game_version = std::string::parse_float(std::format("{}.{}", header_start.version_major, header_start.version_minor));
    u32 log_version;
    Meta meta;
    aoe2op::Operation operations[while($ < std::mem::size())];
};


Aoe2Save savegame @ 0x00;


// To further analyze the rec, click on "Sections" below and
// then click on the rightmost button "Export content" and
// save it to a different file.
// You can then use the aoe2header.hexpat pattern to analyze the rec.
