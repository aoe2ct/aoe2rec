use binrw::binrw;
use serde::Serialize;

use crate::Bool;

#[binrw]
#[derive(Serialize, Debug)]
#[br(import(length: u32))]
pub enum ActionData {
    #[br(magic = 0u8)]
    Interact {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 1u8)]
    Stop {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 2u8)]
    AiInteract {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 3u8)]
    Move {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 4u8)]
    Create {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 5u8)]
    AddAttribute {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 6u8)]
    GiveAttribute {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 10u8)]
    AiMove {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 11u8)]
    Resign {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 15u8)]
    Spec {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 16u8)]
    Waypoint {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 18u8)]
    Stance {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 19u8)]
    Guard {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 20u8)]
    Follow {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 21u8)]
    Patrol {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 23u8)]
    Formation {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 27u8)]
    Save {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 31u8)]
    AiWaypoint {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 32u8)]
    Chapter {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 33u8)]
    DeAttackMove {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 35u8)]
    DeUnknown35 {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 37u8)]
    DeUnknown37 {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 38u8)]
    Autoscout {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 39u8)]
    DeUnknown39 {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 41u8)]
    Unknown41 {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 43u8)]
    Unknown43 {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 44u8)]
    Unknown44 {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 45u8)]
    Unknown45 {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 53u8)]
    AiCommand {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 100u8)]
    AiQueue {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 101u8)]
    Research {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 102u8)]
    Build {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 103u8)]
    Game(Game),
    #[br(magic = 105u8)]
    Wall {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 106u8)]
    Delete {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 107u8)]
    AttackGround {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 108u8)]
    Tribute {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 109u8)]
    DeUnknown109 {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 110u8)]
    Repair {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 111u8)]
    Release {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 112u8)]
    Multiqueue {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 114u8)]
    ToggleGate {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 115u8)]
    Flare {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 117u8)]
    Order {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 119u8)]
    Queue {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 120u8)]
    Gatherpoint {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 122u8)]
    Sell {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 123u8)]
    Buy {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 126u8)]
    DropRelic {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 127u8)]
    TownBell {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 128u8)]
    BackToWork {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 129u8)]
    DeQueue {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 130u8)]
    DeUnknown130 {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 131u8)]
    DeUnknown131 {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 135u8)]
    DeUnknown135 {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 140u8)]
    DeUnknown140 {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 196u8)]
    DeUnknown196 {
        #[br(count = length - 1)]
        data: Vec<u8>,
    },
    #[br(magic = 255u8)]
    Achievements {
        #[br(dbg, count = length - 1)]
        data: Vec<u8>,
    },
}

#[binrw]
#[derive(Serialize, Debug)]
pub struct Interact {
    player_id: u8,
    unknown: u16,
    target_id: u32,
    selected: u32,
    x: f32,
    y: f32,
    // "next"/Peek(Bytes(8)),
    // "flags"/If(lambda ctx: check_flags(ctx.next), Bytes(8)),
    #[br(count=selected)]
    unit_ids: Vec<u32>,
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
    // "speed"/If(this.mode == 'speed', Struct(
    //     Padding(4),
    //     "speed"/Float32l,
    //     Padding(1)
    // )),
    // "instant_build"/If(this.mode == 'instant_build', Struct(
    //     Padding(9)
    // )),
    // "quick_build"/If(this.mode == 'quick_build', Struct(
    //     "status"/Flag,
    //     Padding(8),
    // )),
    // "allied_victory"/If(this.mode == 'allied_victory', Struct(
    //     "player_id"/Byte,
    //     "status"/Flag,
    //     Padding(7)
    // )),
    // "cheat"/If(this.mode == 'cheat', Struct(
    //     "cheat_id"/Byte,
    //     Padding(8)
    // )),
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

#[binrw]
#[derive(Serialize, Debug)]
pub struct DeQueue {
    player_id: u8,
    building_type: u16,
    selected: u8,
    unknown: u8,
    unit_type: u16,
    queue_amount: u8,
    unknown2: u8,
    #[br(count=selected)]
    building_ids: Vec<u32>,
}
