import std.mem;
import std.file;
import type.magic;

namespace aoe2op {
  enum ActionType: u8 {
    Interact = 0,
    Stop = 1,
    Move = 3,
    Resign = 11,
    Stance = 18,
    Guard = 19,
    Follow = 20,
    Patrol = 21,
    Formation = 23,
    Chapter = 32,
    AttackMove = 33,
    AutoScout = 38,
    Transform = 41,
    Unknown42 = 42,
    SwitchAttack = 43,
    Unknown44 = 44,
    MultiGatherpoint = 45,
    Research = 101,
    Build = 102,
    Game = 103,
    Wall = 105,
    Delete = 106,
    AttackGround = 107,
    Repair = 110,
    Ungarrison = 111,
    Multiqueue = 112,
    Gate = 114,
    Flare = 115,
    Order = 117,
    Gatherpoint = 120,
    Sell = 122,
    Buy = 123,
    DropRelic = 126,
    TownBell = 127,
    BackToWork = 128,
    DEQueue= 129,
    Unknown140 = 140,
    Tribute = 196,
  };

  struct BasePlayerAction {
    u8 player_id;
    u16 action_length;
  };

  struct Interact: BasePlayerAction {
    u32 target_id;
    float x;
    float y;
    s16 selection;
    s16 unknown3;
    s8 unknown4[4];
    if (selection > -1)
    u32 unit_ids[selection];
  };

  struct Stop: BasePlayerAction {
    u32 selected;
    u32 unit_ids[selected];
  };

  struct Move: BasePlayerAction {
    s32 unknown2;
    float x;
    float y;
    s16 selected;
    s16 unknown3;
    u8 unknown4[4];
    if (selected > -1)
    u32 unit_ids[selected];
  };

  struct Resign: BasePlayerAction {
    u8 unknown1;
  };

  struct Stance: BasePlayerAction {
    u32 selected;
    u32 stance_id;
    u32 unit_ids[selected];
  };

  struct BaseDefensiveCommand: BasePlayerAction {
    u32 selected;
    u32 target_id;
    u32 unit_ids[selected];
  };

  struct Guard: BaseDefensiveCommand {
  };

  struct Follow: BaseDefensiveCommand {
  };

  struct BaseAggressiveCommand: BasePlayerAction {
    u32 selected;
    u8 unknown1[4];
    float x;
    u8 unknown2[36];
    float y;
    u8 unknown3[36];
    u32 unit_ids[selected];
  };

  struct Patrol: BaseAggressiveCommand {
  };

  struct Formation: BasePlayerAction {
    u32 selected;
    u32 formation_id;
    u32 unit_ids[selected];
  };

  struct Chapter: BasePlayerAction {
  };

  struct AttackMove: BaseAggressiveCommand {
  };

  struct AutoScout: BasePlayerAction {
    u32 selected;
    u32 unit_ids[selected];
  };

  struct Transform: BasePlayerAction {
    u32 unit_id;
    u32 y;
  };

  struct Unknown42: BasePlayerAction {
    u32 unit_id;
  };

  struct SwitchAttack: BasePlayerAction {
    u32 selected;
    u32 unknown;
    u32 unit_ids[selected];
  };

  struct Unknown44: BasePlayerAction {
    u32 selected;
    u32 unit_ids[selected];
  };

  struct MultiGatherpoint: BasePlayerAction {
    s32 unit_id;
    float x;
    float y;
    u16 selected;
    u8 unknown1[2];
    u32 unknown2;
    u32 building_ids[selected];
  };

  struct Research: BasePlayerAction {
    u32 building_id;
    u16 selected;
    u16 technology_type;
    u8 unknown1[5];
    u32 selected_building_ids[selected];
  };

  struct Build: BasePlayerAction {
    u32 unit_count;
    float x;
    float y;
    u32 building_type;
    u32 unknown2;
    s32 sprite_id;
    u8 unknown3[4];
    u32 unit_ids[unit_count];
  };

  enum GameCommand: u32 {
    Diplomacy = 0,
    Speed = 1,
    InstantBuild = 2,
    QuickBuild = 4,
    AlliedVictory = 5,
    Cheat = 6,
    Unknown1 = 9,
    Spy = 10,
    Unknown2 = 11,
    FarmQueue = 13,
    FarmUnqueue = 14,
    FarmAutoqueue = 16,
    FishtrapQueue = 17,
    FishtrapUnqueue = 18,
    FishtrapAutoqueue = 19,
    DefaultStance = 20,
  };

  struct DiplomacyGame {
    std::error("DiplomacyGame is not implemented");
    break;
  };

  struct InstantBuild {
    u16 unknown1;
    u32 unknown2[4];
  };

  struct FarmAutoqueue {
    u32 unknown1;
    u32 unknown2[2];
  };

  struct FishtrapAutoqueue {
    u32 unknown1;
    u32 unknown2[2];
  };

  struct DefaultStance {
    u32 unknown1[3];
  };

  struct Game: BasePlayerAction {
    GameCommand mode;
    match(mode) {
      (GameCommand::Diplomacy): DiplomacyGame [[inline]];
      (GameCommand::InstantBuild): InstantBuild [[inline]];
      (GameCommand::FarmAutoqueue): FarmAutoqueue [[inline]];
      (GameCommand::FishtrapAutoqueue): FishtrapAutoqueue [[inline]];
      (GameCommand::DefaultStance): DefaultStance [[inline]];
      (_): std::error(std::format("Unknown game mode: {} at 0x{:X}", u8(mode), $));
    }
  };

  struct Wall: BasePlayerAction {
    s32 selected;
    s16 x1;
    s16 y1;
    s16 x2;
    s16 y2;
    s32 building_type;
    s32 unknown1;
    u8 unknown2[4];
    s32 unit_ids[selected];
  };

  struct Delete: BasePlayerAction {
    u32 unit_id;
  };

  struct AttackGround: BasePlayerAction {
    s32 selected;
    float x;
    float y;
    u8 unknown1[4];
    u32 unit_ids[selected];
  };

  struct Repair: BasePlayerAction {
    u32 selected;
    u32 target_id;
    u8 unknown1[4];
    u32 unit_ids[selected];
  };

  struct Ungarrison: BasePlayerAction {
    u32 selected;
    float x;
    float y;
    s32 target_id;
    u32 unknown2;
    if (game_version >= 66)
    u8 unknown3;
    u32 unit_ids[selected];
  };

  struct Multiqueue: BasePlayerAction {
    u16 unit_type;
    u8 num_buildings;
    u8 queue_amount;
    u32 building_ids[num_buildings];
  };

  struct Gate: BasePlayerAction {
    u32 building_id;
  };

  struct Flare: BasePlayerAction {
    s32 unknown1;
    float x;
    float y;
    u8 player_count;
    u8 receivers[player_count];
  };

  enum OrderType: u8 {
   Pack = 1,
   Unpack = 2,
   Garrison = 5,
  };

  struct Order: BasePlayerAction {
    u32 selected;
    u32 building_id;
    float x;
    float y;
    s32 unknown2;
    s32 unknown3;
    u8 unknown4;
    u16 unknown5[2];
    u32 unit_ids[selected];
  };

  struct Gatherpoint: BasePlayerAction {
    u32 selected;
    float x;
    float y;
    u32 target_id;
    u32 target_type;
    u8 unknown1;
    u32 building_ids[selected];
  };

  struct BaseMarketAction: BasePlayerAction {
    u16 resource_id;
    u16 amount;
    u32 building_id;
  };

  struct Sell: BaseMarketAction {
  };

  struct Buy: BaseMarketAction {
  };

  struct DropRelic: BasePlayerAction {
    u32 object_id;
  };

  struct TownBell: BasePlayerAction {
    u32 building_id;
    u8 mode;
  };

  struct BackToWork: BasePlayerAction {
    u32 unit_id;
  };

  struct DEQueue: BasePlayerAction {
    u16 selected;
    u8 unknown1[4];
    u16 building_type;
    u16 unit_id;
    u16 amount;
    if (game_version >= 66) {
      u32 unkwnown2;
    }
    u32 building_ids[selected];
  };

  struct Unknown140: BasePlayerAction {
    u32 unit_id;
  };

  struct Tribute: BasePlayerAction {
    float wood;
    float food;
    float gold;
    float stone;
    float wood_tax;
    float food_tax;
    float gold_tax;
    float stone_tax;
    u16 unknown[4];
    u8 target_player_id;
  };

  struct Action {
    u32 length;
    ActionType action_type;
    match(action_type) {
      (ActionType::Interact): Interact [[inline]];
      (ActionType::Stop): Stop [[inline]];
      (ActionType::Move): Move [[inline]];
      (ActionType::Resign): Resign [[inline]];
      (ActionType::Stance): Stance [[inline]];
      (ActionType::Guard): Guard [[inline]];
      (ActionType::Follow): Follow [[inline]];
      (ActionType::Patrol): Patrol [[inline]];
      (ActionType::Formation): Formation [[inline]];
      (ActionType::Chapter): Chapter [[inline]];
      (ActionType::AttackMove): AttackMove [[inline]];
      (ActionType::AutoScout): AutoScout [[inline]];
      (ActionType::Transform): Transform [[inline]];
      (ActionType::Unknown42): Unknown42 [[inline]];
      (ActionType::SwitchAttack): SwitchAttack [[inline]];
      (ActionType::Unknown44): Unknown44 [[inline]];
      (ActionType::MultiGatherpoint): MultiGatherpoint [[inline]];
      (ActionType::Research): Research [[inline]];
      (ActionType::Build): Build [[inline]];
      (ActionType::Game): Game [[inline]];
      (ActionType::Wall): Wall [[inline]];
      (ActionType::Delete): Delete [[inline]];
      (ActionType::AttackGround): AttackGround [[inline]];
      (ActionType::Repair): Repair [[inline]];
      (ActionType::Ungarrison): Ungarrison [[inline]];
      (ActionType::Multiqueue): Multiqueue [[inline]];
      (ActionType::Gate): Gate [[inline]];
      (ActionType::Flare): Flare [[inline]];
      (ActionType::Order): Order [[inline]];
      (ActionType::Gatherpoint): Gatherpoint [[inline]];
      (ActionType::Sell): Sell [[inline]];
      (ActionType::Buy): Buy [[inline]];
      (ActionType::DropRelic): DropRelic [[inline]];
      (ActionType::TownBell): TownBell [[inline]];
      (ActionType::BackToWork): BackToWork [[inline]];
      (ActionType::DEQueue): DEQueue [[inline]];
      (ActionType::Unknown140): Unknown140 [[inline]];
      (ActionType::Tribute): Tribute [[inline]];
      (_): {
        std::warning(std::format("Unknown action type: {} at 0x{:X}", u8(action_type), $));
        break;
        u8 action_data[length + 4 -1];
      }
    }
    u32 unknown_end;
    if (action_type == ActionType::Chapter) {
      u32 chapter_end;
      u32 chapter_address;
      u8 chapter_data[chapter_end - $];
    }
  };

  struct Sync {
    u32 time_increment;
    if(std::mem::read_unsigned($, 4) == 0) {
      u8 unknown1[8];
      u32 sync;
      u32 unknown2;
      u32 sequence;
      if (sequence > 0) {
        u8 unknown3[332];
      }
      u8 unknown4[8];
    }
  };

  struct Viewlock {
    float x;
    float y;
    u32 player_id;
  };

  struct Chat {
    u8 unknown1[4];
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
      (OperationType::Postgame): Postgame [[inline]];
      // (OperationType::Postgame): break;
      (_): {
      break;
      std::error(std::format("Unknown action type: {} at 0x{:X}", u32(type), $));
      }
    }
  };
}
