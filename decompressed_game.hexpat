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

struct LobbySettings<auto major, auto minor> {
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

struct VisibilityMap<auto num_tiles> {
    u32 width;
    u32 height;
    u32 visibility[num_tiles];
};

struct MapInfo {
    u32 size_x;
    u32 size_y;
    u32 zone_num;
    MapTile<size_x * size_y> unknown1[zone_num];
    bool all_visible;
    bool fog_of_war;
    Tile tiles[size_x*size_y];
    u32 data_count;
    u32 unknown_count;
    u8 unknown2[112];
    u32 size_x_2;
    u32 size_y_2;
    std::mem::MagicSearch<"\x98\x9E\x00\x00", float>;
};

struct SavedView {
    float camera_x;
    float camera_y;
};

enum Resource : s16 {
  Food,    // 0x00
  Wood,
  Stone,
  Gold,
  Decay = 0x0C,
  Fish = 0x11,
  None = 0xFFFF,
};

struct Sprite {
    u8 sprite_type;
    if (sprite_type == 0) {
        break;
    }
    u16 id;
    u32 x;
    u32 y;
    u16 frame;
    u8 invisible;
    if (sprite_type == 2) {
        u32 animate_interval;
        u32 animate_last;
        u16 last_frame;
        u8 frame_changed;
        u8 frame_looped;
        u8 animate_flag;
        float last_speed;
    }
    u8 order;
    u8 flag;
    u8 count;
};

struct Particle {
    u16 particle_type;
    if (particle_type == 1) {
        DeString name;
        float x;
        float y;
        u8 unknown1[26];
    }
};

struct StaticObject<auto type> {
    u16 object_type;
    u16 sprite;
    s32 garrisoned_in_id;
    float hitpoints;
    u8 object_state;
    bool sleep;
    bool doppelganger;
    bool go_to_sleep;
    u32 object_id;
    u8 facet;
    float x;
    float y;
    float z;
    u16 screen_offset_x;
    u16 screen_offset_y;
    u16 shadow_offset_x;
    u16 shadow_offset_y;
    Resource resource;
    float amount;
    u8 worker_count;
    u8 current_damage;
    u8 damaged_lately_timer;
    u8 under_attack;
    s32 group_id;
    bool roo_already_called;
    u8 unknown1[17];
    u8 unknown2;
    u16 has_object_props;
    if (has_object_props == 1) {
        // TODO
        type::Magic("\xAA\xBB\xCC");
    } else {
        u8 has_sprite_list;
        if (has_sprite_list > 0) {
            Sprite sprites[10000];
        }
    }
    Particle particles[5];
    u8 unknown3[5];
    u8 unknown4[2];
    u8 unknown5[2];
    DeString unknown6;
    DeString unknown7;
    u8 unknown8[2];
    if(type == 10) {
        padding[1];
    }
    u8 unknown9[18];
};

struct AnimatedObject<auto type> : StaticObject<type> {
    float turn_speed;
};

struct Vector {
    float x;
    float y;
    float z;
};

struct PathData {
    u32 id;
    u32 linked_path_type;
    u32 waypoint_level;
    u32 path_id;
    u32 waypoint;
    s32 state;
    float range;
    s32 target_id;
    float pause_time;
    u32 continue_counter;
    u32 flags;
};

struct MovementData {
    Vector velocity;
    Vector acceleration;
};

struct BaseMovingObject<auto type> : AnimatedObject<type> {
    u32 trail_remainder;
    Vector velocity;
    u8 move_byte;
    float angle;
    u32 turn_towards_time;
    u8 waiting_to_move;
    u8 wait_delays_count;
    u8 on_ground;
    u32 num_path_data;
    PathData path_data[num_path_data];
    u32 has_future_path_data;
    if (has_future_path_data > 0) {
        PathData future_path_data;
    }
    u32 has_movement_data;
    if (has_movement_data > 0) {
        MovementData movement_data;
    }
    Vector position;
    Vector orientation_forward;
    Vector orientation_right;
    u32 last_time_move;
    u32 num_user_defined_waypoints;
    Vector user_waypoints[num_user_defined_waypoints];
    u32 has_substitute_position;
    Vector substitute_position;
    u32 consecutive_substitute_count;
};

struct MovingObject<auto type> : BaseMovingObject<type> {
    u8 unknown1[22];
};

struct Gatherpoint {
    u32 exists;
    Vector position;
    s32 object_id;
    s16 unit_type_id;
};

struct MoveAction {
    float range;
};

struct EnterAction {
    u32 first_time;
};

struct AttackAction {
    float range;
    float min_range;
    u16 missile_id;
    u16 frame_delay;
    u16 need_to_attack;
    u16 was_same_owner;
    u8 indirect_fire_flag;
    u16 move_sprite_id;
    u16 fight_sprite_id;
    u16 wait_sprite_id;
    Vector last_target_position;
};

struct MakeAction {
    float work_timer;
};

struct UnitAction {
    u16 action_type;
    if (action_type == 0) {
        break;
    }
    s32 target_object_pointer;
    s32 target_object_pointer2;
    s32 target_object_id;
    s32 target_object_id2;
    Vector position;
    float timer;
    u8 target_moved_state;
    u16 task_id;
    u8 sub_action_value;
    UnitAction sub_actions[10000];
    s16 sprite_id;
    match (action_type) {
     (1): MoveAction move_action;
     (3): EnterAction enter_action;
     (9): AttackAction attack_action;
     (21): MakeAction make_action;
    }
};

struct Action<auto type> : BaseMovingObject<type> {
    u8 waiting;
    u8 command_flag;
    u16 selected_group_info;
    UnitAction actions[10000];
};

struct BaseCombat<auto type> : Action<type> {
    u8 formation_id;
    u8 formation_row;
    u8 formation_col;
    float attack_timer;
    u8 capture_flag;
    u8 multi_unified_points;
    u8 large_object_radius;
    u32 attack_count;
};

struct Attribute {
    u16 type;
    u32 amount;
    u8 flag;
};

struct DamageSprite {
    u16 sprite;
    u16 damage_percent;
    u8 flag;
};

struct BaseUnit {
    u16 name_len;
    u16 id;
    u16 string_id;
    u16 string_id2;
    u16 unit_class;
    u16 standing_sprite_1;
    u16 standing_sprite_2;
    u16 dying_sprite;
    u16 undead_sprite;
    u8 undead_flag;
    u16 hitpoints;
    float los;
    u8 garrison_capacity;
    Vector radius;
    u16 train_sound;
    u16 damage_sound;
    u16 death_spawn;
    u8 sort_number;
    u8 can_be_built_on;
    u16 button_picture;
    u8 hide_in_editor;
    u16 portrait;
    u8 enabled;
    u8 disabled;
    u16 tile_req_x;
    u16 tile_req_y;
    u16 center_tile_req_x;
    u16 center_tile_req_y;
    float construction_radius_x;
    float construction_radius_y;
    u8 elevation_flag;
    u8 fog_flag;
    u16 terrain_restriction_id;
    u8 movement_type;
    u16 attribute_max;
    float attribute_rot;
    u8 area_effect_level;
    u8 combat_level;
    u8 select_level;
    u8 map_draw_level;
    u8 unit_level;
    float multiple_attribute_mod;
    u8 map_color;
    u32 help_string_id;
    u32 help_page_id;
    u32 hotkey_id;
    u8 recyclable;
    u8 track_as_resource;
    u8 create_doppelganger;
    u8 resource_group;
    u8 occlusion_mask;
    u8 obstruction_type;
    u8 selection_shape;
    u32 object_flags;
    u8 civilization;
    u8 attribute_piece;
    Vector outline_radius;
    Attribute attributes[3];
    u8 num_damage_sprites;
    DamageSprite damage_sprites[num_damage_sprites];
    u16 selected_sound;
    u16 death_sound;
    u8 attack_reaction;
    u8 convert_terrain_flag;
    char name[name_len];
    u16 copy_id;
    u16 group;
};

struct AnimatedUnit : BaseUnit {
    float speed;
};

struct MovingUnit : AnimatedUnit {
    u16 move_sprite;
    u16 run_sprite;
    float turn_speed;
    u8 size_class;
    u16 trailing_unit;
    u8 trailing_options;
    float trailing_spacing;
    u8 move_algorithm;
    float turn_radius;
    float turn_radius_speed;
    float maximum_yaw_per_second_moving;
    float stationary_yaw_per_revolution_time;
    float maximum_yaw_per_second_stationary;
};

struct ActionUnit : MovingUnit {
    u16 default_task;
    float search_radius;
    float work_rate;
    float handicap_work_rate;
    u16 drop_site;
    u16 backup_drop_site;
    u8 task_by_group;
    u16 command_sound;
    u16 move_sound;
    u8 run_pattern;
};

struct Weapon {
    u16 type;
    u16 value;
};

struct BaseCombatUnit : ActionUnit {
    u16 base_armour;
    u16 num_weapons;
    Weapon weapons[num_weapons];
    u16 num_armours;
    Weapon armours[num_armours];
    u16 defense_terrain_bonus;
    float weapon_max_range;
    float area_effect_range;
    float attack_speed;
    u16 missile_id;
    u16 base_hit_chance;
    u8 break_off_combat;
    u16 frame_delay;
    Vector weapon_offset;
    u8 blast_level_offense;
    float weapon_range_min;
    float missed_missile_spread;
    u16 fight_sprite;
    u16 displayed_armour;
    u16 displayed_attack;
    float displayed_range;
    float displayed_reload_time;
};

struct MissileUnit : BaseCombatUnit {
    u8 missile_type;
    u8 targeting_type;
    u8 missile_hit_info;
    u8 missile_die_info;
    u8 area_effect_specials;
    float ballistics_raton;
};

struct AttributeCost {
    u16 type;
    u16 amount;
    u8 flag;
    u8 unknown;
};

struct CombatUnit : BaseCombatUnit {
    AttributeCost cost[3];
    u16 create_time;
    u16 create_at_building;
    u8 create_button;
    float rear_attack_modifier;
    float flank_attack_modifier;
    u8 tribe_unit_type;
    u8 hero_flag;
    u32 garrison_sprite;
    float volley_fire_amount;
    u8 max_attacks_in_volley;
    float volley_spread_x;
    float volley_spread_y;
    float volley_start_spread_adjustment;
    u32 volley_missile;
    u32 special_attack_sprite;
    u8 special_attack_flag;
    u16 displayed_pierce_armour;
};

struct LinkedBuilding {
    u16 unit_id;
    float x_offset;
    float y_offset;
};

struct BuildingUnit : CombatUnit {
    u16 construction_sprite;
    u16 snow_sprite;
    u8 connect_flag;
    u16 facet;
    u8 destroy_on_build;
    u16 on_build_make_unit;
    u16 on_build_make_tile;
    u16 on_build_make_overlay;
    u16 on_build_make_tech;
    u8 can_burn;
    LinkedBuilding linked_buildings[4];
    u16 construction_unit;
    u16 transform_unit;
    u16 transform_sound;
    u16 construction_sound;
    u8 garrison_byte;
    float garrison_heal_rate;
    float garrison_repair_rate;
    u16 salvage_unit;
    u8 salvage_attributes[6];
};

struct Unit {
    u8 type;
    match (type) {
        (10): BaseUnit [[inline]];
        (15): BaseUnit [[inline]];
        (20): AnimatedUnit [[inline]];
        (25): AnimatedUnit [[inline]];
        (30): MovingUnit [[inline]];
        (40): ActionUnit [[inline]];
        (50): BaseCombatUnit [[inline]];
        (60): MissileUnit [[inline]];
        (70): CombatUnit [[inline]];
        (80): BuildingUnit [[inline]];
    }
};

struct Waypoint {
    Vector position;
    u8 facet_to_next_waypoint;
    u8 unknown[3];
};

struct Order {
    u32 issuer;
    u32 type;
    u32 priority;
    u32 target_id;
    u32 target_player;
    Vector target_location;
    float range;
};

struct Notification {
    u32 caller;
    u32 recipient;
    u32 notification_type;
    u32 params[3];
};

struct OrderHistory {
    u32 order;
    u32 action;
    u32 time;
    Vector position;
    u32 target_id;
    u32 target_attack_category;
    Vector target_position;
};

struct Retarget {
    u32 target_id;
    u32 retarget_timeout;
};

struct UnitAI {
    s32 mood;
    s32 current_order;
    s32 current_order_priority;
    s32 current_action;
    s32 current_target;
    s16 current_target_type;
    u8 unknown1[2];
    Vector current_target_location;
    float desired_target_distance;
    s32 last_action;
    s32 last_order;
    s32 last_target;
    s32 last_target_type;
    s32 last_update_type;
    u32 idle_timer;
    u32 idle_timeout;
    u32 adjusted_idle_timeout;
    u32 secondary_timer;
    u32 look_around_timer;
    u32 look_around_timeout;
    s32 defend_target;
    float defense_buffer;
    Waypoint last_world_position;
    u32 num_orders;
    Order orders[num_orders];
    u32 num_notifications;
    Notification notifications[num_notifications];
    u32 num_attacking_units;
    u32 attacking_units[num_attacking_units];
    u8 stop_after_target_killed;
    u8 state;
    float state_position_x;
    float state_position_y;
    u32 time_since_enemy_sighting;
    u8 alert_mode;
    s32 alert_mode_object_id;
    u32 has_patrol_path;
    if (has_patrol_path > 0) {
        // This is empty?
    }
    u32 patrol_current_waypoint;
    u32 num_order_history;
    OrderHistory order_history[num_order_history];
    u32 last_retarget_time;
    u32 randomized_traget_timer;
    u32 num_retarget_entries;
    Retarget retarget_entries[num_retarget_entries];
    s32 best_unit_to_attack;
    u8 formation_type;
    u8 unknown_de1[4];
    u8 unknown_de2;
};

fn has_position() {
    auto yada = std::mem::read_unsigned($, 5);
    u8 bytes[5] = {0x00, 0xFF, 0xFF, 0xFF, 0xFF};
    for (u8 i = 0, i < 5, i = i + 1) {
        if(std::mem::read_unsigned($ + i, 1) != bytes[i]) {
            return true;
        }
    }
    return false;
};

struct Combat<auto type> : BaseCombat<type> {
    u8 unknown_de1[14];
    u8 unknown_de2[16];
    u8 unknown_de3[1];
    u8 unknown_de4[4];
    u8 next_volley;
    u8 using_special_animation;
    u8 own_base;
    if (own_base > 0) {
        Unit base_unit;
    }
    u16 attribute_amounts[6];
    u16 decay_timer;
    u32 raider_build_countdown;
    u32 locked_down_count;
    u8 inside_garrison_count;
    u32 has_ai;
    if (has_ai > 0) {
        UnitAI ai;
    }

    if(has_position()) {
        Vector position;
        u8 flag;
    }
    u8 town_bell_flag;
    s32 town_bell_target_id;
    float town_bell_target_x;
    float town_bell_target_y;
    s32 town_bell_target_id2;
    s32 town_bell_target_type;
    s32 town_bell_action;
    float berserker_timer;
    u8 num_builders;
    u8 num_healers;
    u32 unknown_de5;
    u32 unknown_de6;
    u8 unknown_de7[4];
    u8 unknown_de8[48];
    u8 unknown_de9[44];
};

struct ProductionUnit {
    u16 unit_type_id;
    u16 count;
};

struct Building<auto type> : Combat<type> {
    u8 built;
    float build_points;
    u32 unique_building_id;
    u8 culture;
    u8 burning;
    u32 last_burn_time;
    u32 last_garrison_time;
    u32 relic_count;
    u32 specific_relic_count;
    Gatherpoint gatherpoint;
    u8 desolid_flag;
    u32 pending_order;
    s32 linked_owner;
    s32 linked_children[3];
    u8 captured_unit_count;
    UnitAction extra_actions[10000];
    u16 capacity;
    ProductionUnit production_queue[capacity];
    u16 size;
    u16 production_queue_total_units;
    u8 production_queue_enabled;
    UnitAction production_queue_actions[10000];
    Vector endpoint;
    Vector endpoint_2;
    u32 gate_locked;
    u32 first_update;
    u32 close_timer;
    u8 terran_type;
    u8 semi_asleep;
    u8 snow_flag;
    u8 unknown_de_flag;
    u16 unknown_de1;
    u8 unknown_de2;
    u8 unknown_de3[4];
    u8 unknown_de4[4];
    u8 unknown_de5[12];
};


struct Object {
    u8 type;
    u8 player_id;
    match (type) {
        (10): StaticObject<type> [[inline]];
        (30): MovingObject<type> [[inline]];
        (70): Combat<type> [[inline]];
        (80): Building<type> [[inline]];
        (0): continue;
        (_): break;
    }
};

struct ObjectsDelimiter {
    u8 unknown1;
    u32 unknown2;
    u32 unknown3;
};

struct KnownPlayerAttributes {
};

struct Init<auto num_players> {
    auto init_start_position = $;
    u8 player_type;
    u8 unknown1;
    u8 other_player_diplomacy[num_players];
    u32 player_stances[num_players];
    u32 allied_vision;
    bool allied_victory;
    u16 name_length;
    std::string::NullString name;
    auto init_before_marker = $;
    type::Magic<"\x16">;
    u32 num_player_attributes;
    type::Magic<"\x21">;
    float attrs[num_player_attributes];
    float unknown_extra_attrs[num_player_attributes];
    type::Magic<"\x0B">;
    SavedView camera;
    s32 num_saved_views;
    if (num_saved_views > 0) {
      SavedView saved_views[num_saved_views];
    }
    u16 spawn_x;
    u16 spawn_y;
    u8 culture;
    u8 civilization;
    bool resigned;
    u8 unknown2;
    u8 player_color;
    u8 unknown3;
    $ = std::mem::find_string_in_range(0, $, std::mem::size(), "\x0B\x00\x08\x00\x00\x00\x02\x00\x00") + 9;
    Object objects[while(std::mem::read_unsigned($, 1) != 0x00)];
    type::Magic<"\x00\x0B">;
    if(player_type != 2) {
        u32 sleeping_size;
        u32 sleeping_grow;
        Object sleeping_objects[while(std::mem::read_unsigned($, 1) != 0x00)];
        type::Magic<"\x00\x0B">;
        u32 doppel_size;
        u32 doppel_grow;
        Object doppel_objects[while(std::mem::read_unsigned($, 1) != 0x00)];
        type::Magic<"\x00\x0B">;
    }
    std::print(std::format("0x{:X}", $));
    go_to_next_player(num_player_attributes, init_before_marker - init_start_position);
    break;
 };
 
 fn go_to_next_player(u32 num_player_attributes, u32 backtrack) {
     auto loc = std::mem::find_sequence_in_range(1, std::mem::base_address(), std::mem::size(), 0x16, u8(num_player_attributes));
     if (loc > std::mem::size()) {
      std::print("Not found");
     }
     std::print(std::format("0x{:X}", $));
     std::print(std::format("0x{:X}", loc - backtrack - 3));
     str next_player = std::format("{:>X}", num_player_attributes);
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
    LobbySettings<version_major, version_minor> lobby_settings;
    AIConfig ai_config;
    Replay replay;
    MapInfo map_info;
    Init<replay.num_players> players[replay.num_players];
};

Rec savegame @ 0x00;