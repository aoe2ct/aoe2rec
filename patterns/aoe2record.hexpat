#pragma endian little
#pragma pattern_limit 10000000
#pragma array_limit 1000000
import std.mem;
// import std.file;
import hex.dec;
import hex.provider;
// import type.magic;
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

struct Aoe2Save {
    u32 hlen;
    u32 check;
    u8 compressed[hlen - 8];
    u32 log_version;
    Meta meta;
    aoe2op::Operation operations[while($ < std::mem::size())];
};


Aoe2Save savegame @ 0x00;

std::mem::Section header = std::mem::create_section("Header");
hex::dec::zlib_decompress(savegame.compressed, header, -15);

// To further analyze the rec, click on "Sections" below and
// then click on the rightmost button "Export content" and
// save it to a different file.
// You can then use the aoe2header.hexpat pattern to analyze the rec.
