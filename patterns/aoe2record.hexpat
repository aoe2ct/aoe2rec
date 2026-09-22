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
import aoe2header;

struct HeaderStart {
    u8 rec_version[7];
    type::Magic<"\x00">;
    float checker;
    u16 version_minor;
    u16 version_major;
};

struct Aoe2Save {
    u32 header_end;
    u32 next_chapter;
    std::print(header_end);
    u8 compressed[while($ < header_end)];
    std::mem::Section header = std::mem::create_section("Header");
    hex::dec::zlib_decompress(compressed, header, -15);
    HeaderStart header_start @ 0x0 in header;
    // Rec parsed_header @ 0x0 in header;
    game_version = std::string::parse_float(std::format("{}.{}", header_start.version_major, header_start.version_minor));
    if (next_chapter > 0) {
      aoe2op::Operation operations[while($ < next_chapter)];
    } else  {
      aoe2op::Operation operations[while($ < std::mem::size())];
    }
};


Aoe2Save savegame[while(std::mem::read_signed($, 4) > 0)] @ 0x00;


// To further analyze the rec, click on "Sections" below and
// then click on the rightmost button "Export content" and
// save it to a different file.
// You can then use the aoe2header.hexpat pattern to analyze the rec.
