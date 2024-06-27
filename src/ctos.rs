use deku::prelude::*;
use crate::common::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
#[repr(u8)]
pub enum CTOSMsg {
    #[deku(id = 0x01)]
    Response = 0x01,
    #[deku(id = 0x02)]
    UpdateDeck = 0x02,
    #[deku(id = 0x03)]
    RPSChoice = 0x03,
    #[deku(id = 0x04)]
    TurnChoice = 0x04,
    #[deku(id = 0x10)]
    PlayerInfo(PlayerInfo) = 0x10,
    #[deku(id = 0x11)]
    CreateGame(CreateGame) = 0x11,
    #[deku(id = 0x12)]
    JoinGame(JoinGame) = 0x12,
    #[deku(id = 0x13)]
    LeaveGame = 0x13,
    #[deku(id = 0x14)]
    Surrender = 0x14,
    #[deku(id = 0x15)]
    TimeConfirm = 0x15,
    #[deku(id = 0x16)]
    Chat = 0x16,
    #[deku(id = 0x20)]
    ToDuelist = 0x20,
    #[deku(id = 0x21)]
    ToObserver = 0x21,
    #[deku(id = 0x22)]
    Ready = 0x22,
    #[deku(id = 0x23)]
    NotReady = 0x23,
    #[deku(id = 0x24)]
    Kick(Kick) = 0x24,
    #[deku(id = 0x25)]
    Start = 0x25,
    #[deku(id = 0xF0)]
    Rematch(Rematch) = 0xF0
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct PlayerInfo {
    pub name: [u16; 20],
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct CreateGame {
    pub info: HostInfo,
    pub name: [u16; 20],
    pub password: [u16; 20],
    pub notes: [u8; 200],
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct JoinGame {
    pub version: u16,
    pub game_id: u32,
    pub password: [u16; 20],
    pub version_2: ClientVersion,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct Kick {
    pub pos: u8
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct Rematch {
    pub rematch: u8
}