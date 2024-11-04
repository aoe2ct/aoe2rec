#pragma endian little
#include <std/mem.pat>
#include <std/file.pat>
#include <hex/provider.pat>
import hex.dec;

u32 hlen @ 0x00;
u32 check @ 0x04;
u8 compressed[hlen] @ 0x08;


std::mem::Section header = std::mem::create_section("Header");
hex::dec::zlib_decompress(compressed, header, -15);

// To further analyze the rec, click on "Sections" below and
// then click on the rightmost button "Export content" and
// save it to a different file.
// You can then use the decompressed.hexpat pattern to analyze the rec.