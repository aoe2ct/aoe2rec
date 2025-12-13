use binrw::binrw;
use serde::Serialize;

use crate::Bool;

#[binrw]
#[derive(Serialize, Debug)]
#[br(import(major: u16))]
pub enum ActionData {
    #[br(magic = 0u8)]
    Interact {
        player_id: u8,
        action_length: u16,
        target_id: u32,
        x: f32,
        y: f32,
        selected: i16,
        unknown1: i16,
        unknown2: [i8; 4],
        #[br(count = if selected > -1 { selected } else { 0 })]
        unit_ids: Vec<u32>,
    },
    #[br(magic = 1u8)]
    Stop { player_id: u8, action_length: u16 },
    #[br(magic = 2u8)]
    AiInteract { player_id: u8, action_length: u16 },
    #[br(magic = 3u8)]
    Move {
        player_id: u8,
        action_length: u16,
        unknown1: i32,
        x: f32,
        y: f32,
        selected: i16,
        unknown2: i16,
        unknown3: [u8; 4],
        #[br(count = if selected > -1 { selected } else { 0 })]
        unit_ids: Vec<u32>,
    },
    #[br(magic = 4u8)]
    Create { player_id: u8, action_length: u16 },
    #[br(magic = 5u8)]
    AddAttribute { player_id: u8, action_length: u16 },
    #[br(magic = 6u8)]
    GiveAttribute { player_id: u8, action_length: u16 },
    #[br(magic = 10u8)]
    AiMove { player_id: u8, action_length: u16 },
    #[br(magic = 11u8)]
    Resign {
        player_id: u8,
        action_length: u16,
        #[br(count = action_length)]
        data: Vec<u8>,
    },
    #[br(magic = 15u8)]
    Spec { player_id: u8, action_length: u16 },
    #[br(magic = 16u8)]
    Waypoint { player_id: u8, action_length: u16 },
    #[br(magic = 18u8)]
    Stance { player_id: u8, action_length: u16 },
    #[br(magic = 19u8)]
    Guard { player_id: u8, action_length: u16 },
    #[br(magic = 20u8)]
    Follow { player_id: u8, action_length: u16 },
    #[br(magic = 21u8)]
    Patrol { player_id: u8, action_length: u16 },
    #[br(magic = 23u8)]
    Formation { player_id: u8, action_length: u16 },
    #[br(magic = 27u8)]
    Save { player_id: u8, action_length: u16 },
    #[br(magic = 31u8)]
    AiWaypoint { player_id: u8, action_length: u16 },
    #[br(magic = 32u8)]
    Chapter { player_id: u8, action_length: u16 },
    #[br(magic = 33u8)]
    DeAttackMove { player_id: u8, action_length: u16 },
    #[br(magic = 35u8)]
    DeUnknown35 { player_id: u8, action_length: u16 },
    #[br(magic = 37u8)]
    DeUnknown37 { player_id: u8, action_length: u16 },
    #[br(magic = 38u8)]
    Autoscout { player_id: u8, action_length: u16 },
    #[br(magic = 39u8)]
    DeUnknown39 { player_id: u8, action_length: u16 },
    #[br(magic = 41u8)]
    Unknown41 { player_id: u8, action_length: u16 },
    #[br(magic = 43u8)]
    SwitchAttack { player_id: u8, action_length: u16 },
    #[br(magic = 44u8)]
    Unknown44 { player_id: u8, action_length: u16 },
    #[br(magic = 45u8)]
    Unknown45 { player_id: u8, action_length: u16 },
    #[br(magic = 53u8)]
    AiCommand { player_id: u8, action_length: u16 },
    #[br(magic = 100u8)]
    AiQueue { player_id: u8, action_length: u16 },
    #[br(magic = 101u8)]
    Research {
        player_id: u8,
        action_length: u16,
        building_id: i32,
        selected: i16,
        technology_type: u16,
        unknown1: [u8; 5],
        #[br(count = if selected > -1 { selected } else { 0 })]
        building_ids: Vec<i32>,
    },
    #[br(magic = 102u8)]
    Build { player_id: u8, action_length: u16 },
    #[br(magic = 103u8)]
    Game(Game),
    #[br(magic = 104u8)]
    Unknown104 { player_id: u8, action_length: u16 },
    #[br(magic = 105u8)]
    Wall { player_id: u8, action_length: u16 },
    #[br(magic = 106u8)]
    Delete { player_id: u8, action_length: u16 },
    #[br(magic = 107u8)]
    AttackGround { player_id: u8, action_length: u16 },
    #[br(magic = 108u8)]
    Tribute { player_id: u8, action_length: u16 },
    #[br(magic = 109u8)]
    DeUnknown109 { player_id: u8, action_length: u16 },
    #[br(magic = 110u8)]
    Repair { player_id: u8, action_length: u16 },
    #[br(magic = 111u8)]
    Release { player_id: u8, action_length: u16 },
    #[br(magic = 112u8)]
    Multiqueue { player_id: u8, action_length: u16 },
    #[br(magic = 114u8)]
    ToggleGate { player_id: u8, action_length: u16 },
    #[br(magic = 115u8)]
    Flare { player_id: u8, action_length: u16 },
    #[br(magic = 117u8)]
    Order {
        player_id: u8,
        action_length: u16,
        selected: i16,
        unknown1: i16,
        building_id: i32,
        unknown2: f32,
        unknown3: f32,
        unknown4: i32,
        unknown5: u32,
        order_type: OrderType,
        multiple: u16,
        unknown7: u16,
        #[br(count = if selected > -1 { selected } else { 0 })]
        object_ids: Vec<u32>,
    },
    #[br(magic = 119u8)]
    Queue { player_id: u8, action_length: u16 },
    #[br(magic = 120u8)]
    Gatherpoint { player_id: u8, action_length: u16 },
    #[br(magic = 122u8)]
    Sell { player_id: u8, action_length: u16 },
    #[br(magic = 123u8)]
    Buy { player_id: u8, action_length: u16 },
    #[br(magic = 126u8)]
    DropRelic { player_id: u8, action_length: u16 },
    #[br(magic = 127u8)]
    TownBell { player_id: u8, action_length: u16 },
    #[br(magic = 128u8)]
    BackToWork { player_id: u8, action_length: u16 },
    #[br(magic = 129u8)]
    DeQueue {
        player_id: u8,
        action_length: u16,
        selected: u16,
        unknown1: [u8; 4],
        building_type: u16,
        unit_id: u16,
        amount: u16,
        #[br(if(major >= 66))]
        unknown2: u32,
        #[br(count = selected)]
        building_ids: Vec<u32>,
    },
    #[br(magic = 130u8)]
    DeUnknown130 { player_id: u8, action_length: u16 },
    #[br(magic = 131u8)]
    DeUnknown131 { player_id: u8, action_length: u16 },
    #[br(magic = 135u8)]
    DeUnknown135 { player_id: u8, action_length: u16 },
    #[br(magic = 140u8)]
    DeUnknown140 { player_id: u8, action_length: u16 },
    #[br(magic = 196u8)]
    DeUnknown196 { player_id: u8, action_length: u16 },
    #[br(magic = 255u8)]
    Achievements { player_id: u8, action_length: u16 },
}

#[binrw]
#[derive(Serialize, Debug)]
#[brw(repr(u8))]
pub enum OrderType {
    Pack = 1,
    Unpack = 2,
    Unqueue = 4,
    UnknownOrder5 = 5,
    Garrison = 6,
    UnknownOrder8 = 8,
}

#[binrw]
#[derive(Serialize, Debug)]
pub enum Game {
    #[br(magic = 0u8)]
    Diplomacy {
        #[br(pad_after = 1)]
        player_id: u8,
        #[br(pad_after = 3)]
        target_player_id: u8,
        stance_float: f32,
        stance: u8,
    },
    #[br(magic = 1u8)]
    Speed {
        #[br(pad_after = 1)]
        player_id: u8,
        unknown: u32,
        speed: f32,
        unknown2: u8,
    },
    #[br(magic = 2u8)]
    InstantBuild {
        #[br(pad_after = 1)]
        player_id: u8,
        unknown: [u8; 9],
        uknown2: [u8; 3],
        uknown3: u32,
    },
    #[br(magic = 3u8)]
    UnknownCommand3 {
        #[br(pad_after = 1)]
        player_id: u8,
    },
    #[br(magic = 4u8)]
    QuickBuild {
        #[br(pad_after = 1)]
        player_id: u8,
        status: Bool,
    },
    #[br(magic = 5u8)]
    AlliedVictory {
        #[br(pad_after = 1)]
        player_id: u8,
        status: Bool,
    },
    #[br(magic = 6u8)]
    Cheat {
        #[br(pad_after = 1)]
        player_id: u8,
        cheat_id: u8,
    },
    #[br(magic = 7u8)]
    UnknownCommand4 {
        #[br(pad_after = 1)]
        player_id: u8,
    },
    #[br(magic = 8u8)]
    UnknownCommand5 {
        // This seems to be something map scripts do at the beginning of the game.
        // For example this seems to happen on team Arena and Black Forest
        #[br(pad_after = 1)]
        player_id: u8,
    },
    // "unk0"/If(this.mode == 'unk0', Struct(
    //     Padding(9)
    // )),
    // "spy"/If(this.mode == 'spy', Struct(
    //     Padding(9)
    // )),
    // "unk1"/If(this.mode == 'unk1', Struct(
    //     Padding(9)
    // )),
    // "farm_queue"/If(this.mode == 'farm_queue', Struct(
    //     "amount"/Byte, # this seems to be a bit inconsistent between versions, needs more research
    //     Padding(8)
    // )),
    // "farm_unqueue"/If(this.mode == 'farm_unqueue', Struct(
    //     "amount"/Byte, # this seems to be a bit inconsistent between versions, needs more research
    //     Padding(8)
    // )),
    // # toggle farm auto seed queue
    // "farm_autoqueue"/If(this.mode == 'farm_autoqueue', Struct(
    //     Padding(9)
    // )),
    //
    // "fishtrap_queue" / If(this.mode == 'fishtrap_queue', Struct(
    //     "amount" / Byte,
    //     Padding(8)
    // )),
    // "fishtrap_unqueue" / If(this.mode == 'fishtrap_unqueue', Struct(
    //     "amount" / Byte,
    //     Padding(8)
    // )),
    //
    // # toggle fish trap auto place queue
    // "fishtrap_autoqueue"/If(this.mode == 'fishtrap_autoqueue', Struct(
    //     Padding(9)
    // )),
    //
    // # toggles the default stance when units are created. All players start on aggressive by default, if the player
    // # (initially) has defensive enabled it is called right before the first unit is queued, and again every time
    // # the player toggles it in the game options menu
    // "default_stance" / If(this.mode == 'default_stance', Struct(
    //     Padding(9)
    // )),
    // Padding(3)
}
