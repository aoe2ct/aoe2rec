#pragma endian little
#pragma pattern_limit 10000000000
#pragma array_limit 70000
import type.magic;

struct Separator {
    type::Magic<"\xA3\x5F\x02\x00">;
};

struct DeString {
    type::Magic<"\x60\x0A">;
    std::string::SizedString<u16> string;
};

struct DeStringArray {
    u32 crc;
    if (crc > 0 && crc < 255)
        break;
    
    DeString string;
};

struct Player {
    u32 dlc_id;
    s32 color_id;
    u8 selected_color;
    u8 selected_team_id;
    u8 resolved_team_id;
    u8 dat_crc[8];
    u8 mp_game_version;
    u32 civ_id;
    u32 custom_civ_count;
    u32 custom_civ_ids[custom_civ_count];
    DeString ai_type;
    u8 ai_civ_name_index;
    DeString ai_name;
    DeString name;
    u32 player_type;
    u32 profile_id;
    u8 ai_unknown[4];
    s32 player_number;
    bool prefer_random;
    bool custom_ai;
    u8 handicap[8];
};

struct EmptySlot {
    u32 i0x;
    u32 i0a;
    u32 i0b;
    u32 unknown1;
    DeString s1;
    u8 a2;
    DeString s2;
    DeString s3;
    u8 a3[22];
    u32 i1;
    u32 i2;
    u8 a4[8];
};

struct AIFile {
    u32 unknown;
    DeString name;
    u32 unknown2;
};

struct GameOptions<auto major, auto minor> {
    u32 game_options_version;
    u32 num_dlc;
    u32 dlcs[num_dlc];
    u32 dataset_ref;
    u32 difficulty;
    u32 selected_map_id;
    u32 resolved_map_id;
    u32 reveal_map;
    u32 victory_type_id;
    u32 starting_resources_id;
    u32 starting_age_id;
    u32 ending_age_id;
    u32 game_type;
    Separator;
    Separator;
    float speed;
    u32 treaty_length;
    u32 population_limit;
    u32 n_players;
    u32 unused_player_color;
    s32 victory_amount;
    u8 unknown1;
    Separator;
    bool trade_enabled;
    bool team_bonus_disabled;
    bool random_positions;
    bool all_techs;
    u8 num_starting_units;
    bool lock_teams;
    bool lock_speed;
    bool multiplayer;
    bool cheats;
    bool record_game;
    bool animals_enabled;
    bool predators_enabled;
    bool turbo_enabled;
    bool shared_exploration;
    bool team_positions;
    u32 sub_game_mode;
    u32 battle_royale_time;
    bool handicap;
    bool unknown2;
    Separator;
    Player players[n_players];
    u8 unknown3[9];
    bool fog_of_war;
    bool cheat_notification;
    bool colored_chat;
    EmptySlot empty_slots[8 - n_players];
    Separator;
    bool ranked;
    bool allow_spec;
    u32 lobby_visibility;
    bool hidden_civs;
    bool matchmaking;
    u32 spec_delay;
    bool scenario_civ;
    DeStringArray rms_strings[1000];
    DeStringArray other_strings1[100];
    DeStringArray other_strings2[100];
    DeStringArray other_strings3[100];
    DeStringArray other_strings4[100];
    DeStringArray other_strings5[100];
    DeStringArray other_strings6[100];
    DeStringArray other_strings7[100];
    DeStringArray other_strings8[100];
    DeStringArray other_strings9[100];
    DeStringArray other_strings10[100];
    DeStringArray other_strings11[100];
    DeStringArray other_strings12[100];
    DeStringArray other_strings13[100];
    DeStringArray other_strings14[100];
    DeStringArray other_strings15[100];
    DeStringArray other_strings16[100];
    DeStringArray other_strings17[100];
    DeStringArray other_strings18[100];
    DeStringArray other_strings19[100];
    DeStringArray other_strings20[100];
    DeStringArray other_strings21[100];
    DeStringArray other_strings22[100];
    u32 num_strategic_numbers;
    s32 strategic_numbers[num_strategic_numbers];
    u32 num_ai_files;
    AIFile ai_files[num_ai_files];
    u32 unknown4;
    u32 unknown5;
    u32 unknown6;
    u8 guid[16];
    DeString lobby_name;
    u8 unknown7[8];
    DeString modded_dataset;
    u8 unknown8[19];
    u8 unknown9[5];
    u8 unknown10[3];
    DeString unknown11;
    u8 unknown12[3];
    u8 unknown13;
    u8 unknown14[8];
    u8 unknown15[21];
    u8 unknown16[4];
    u8 unknown17[8];
    u8 unknown18[3];
    u8 unknown19[8];
    if (major >= 63)
    u8 unknown24[5];
    DeString unknown20;
    u8 unknown21[5];
    u8 unknown22;
    u8 unknown23[2];
    u32 ver37[2];
};

struct UnknownAI {
    u16 seq;
    type::Magic<"\xFF\xFF">;
    u8 unknown1;
    u8 unknown_count;
    type::Magic<"\x00\x00">;
    u32 unknown2[unknown_count * 6];
    u32 unknown3;
    u32 unknown4;
    u32 unknown5;
    s32 crc;
    if (crc == -1) {
        break;
    }
};

struct AIConfig {
    u32 has_ai;
    if (has_ai == 1) {
        u16 max_strings;
        u16 strings_count;
        u32 unknown1;
        std::string::SizedString<u32> ai_strings[strings_count];
        u8 unknown3[3];
        u16 unknown4;
        u16 unknown5;
        u32 unknown6[2];
        u16 unknown_max;
        u16 unknown_count;
        
        u32 unknown7[3];
        UnknownAI unknown8[unknown_count];        
        u16 unknown_max2;
        u16 unknown_count2;
        u32 unknown9[3];
        UnknownAI unknown10[unknown_count2];
        u16 unknown_max3;
        u16 unknown_count3;
        u32 unknown11[3];
        UnknownAI unknown12[unknown_count3];
        type::Magic<"\x00"> void[4096];
    }
};

struct Replay {
    u32 old_time;
    u32 world_time;
    u32 old_world_time;
    u32 game_speed_id;
    u32 world_time_delta_seconds;
    float timer;
    float game_speed;
    bool temp_pause;
    u32 next_object_id;
    s32 next_reusable_object_id;
    u32 random_seed;
    u32 random_seed_2;
    u16 rec_player;
    u8 num_players;
    bool instant_build;
    bool cheats_enabled;
    u16 game_mode;
    u32 campaign;
    u32 campaign_player;
    u32 campaign_scenario;
    u32 king_campaign;
    u8 king_campaign_player;
    u8 king_campaign_scenario;
    u32 player_turn;
    u32 player_turns[num_players];
    padding[8];
};

struct MapTile<auto parent_tile_num> {
    u32 tile_num;
    u8 unknown1[2044];
    u16 unknown_tiles[parent_tile_num];
    u32 num_floats;
    float unknown2[num_floats];
    u32 unknown3;
};

struct Tile {
    u8 terrain_type;
    u8 unknown1;
    u8 original_terrain_type;
    u8 elevation;
    u16 unknown2;
    u16 unknown3;
    u16 unknown4;
    if (terrain_type == 255) {
        u8 terrain_type;
        u8 elevation2;
        padding[1];
    }
};

struct Obstruction {
    u32 obstructions_count;
    padding[8 * obstructions_count];
};

struct VisibilityMap<auto num_tiles> {
    u32 width;
    u32 height;
    u32 visibility[num_tiles];
};

struct MapInfo {
    u32 size_x;
    u32 size_y;
    u32 zone_num;
    break;
    MapTile<size_x * size_y> unknown1[zone_num];
    bool all_visible;
    bool fog_of_war;
    Tile tiles[size_x*size_y];
    u32 data_count;
    u32 unknown_count;
    u8 unknown2[112];
    //Obstruction obstructions[data_count];
    u32 size_x_2;
    u32 size_y_2;
    u8 todo[115212];
};

struct Init<auto num_players> {
    u8 type_player_1;
    u8 unknown_player_1;
    u8 other_player_diplomacy[num_players];
    u32 player_stances[num_players];
    u32 allied_vision;
    bool allied_victory;
    u16 name_length;
    std::string::NullString name;
    type::Magic<"\x16">;
    u32 num_player_attributes;
    type::Magic<"\x21">;
    float attrs[num_player_attributes];
};

struct Rec {
    std::string::NullString rec_version;
    float checker;
    u16 version_minor;
    u16 version_major;
    u32 build;
    s32 timestamp;
    u16 version[2];
    u16 interval_version[2];
    GameOptions<version_major, version_minor> game_options;
    AIConfig ai_config;
    Replay replay;
    MapInfo map_info;
    break;
    Init<replay.num_players> players[replay.num_players];
};

struct DK2 {
 u8 unknown1[18];
 DeString string1;
 DeString string2;
 u8 unknown2[32];
};

Rec savegame @ 0x00;


//Init init1 @ 0x6966F0;


//Init init_gaia @ 0x1978FB;
//Init init_2 @ 0x7419FE;
//Init init_3 @ 0x7ED060;
//Init init_4 @ 0x89A5CC;
//u32 num_blocks_1 @ 0xE290D;
//DK2 dk3[num_blocks_1] @ $;
//u32 num_blocks_2 @ 0x89B9CD;
//DK2 dk2[num_blocks_2] @ $;
//u32 u32_at_0xC62B[2647] @ 0xC567;

// bytes = 112
// b8 = 14
// map_size = 120
// n_players = 2
// zone_num = 6
// data_count = 3
// unknown_count = 8
// after_size = 115212
//
//Init<3> init_at_0x232189 @ 0x232189;