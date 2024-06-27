use deku::prelude::*;
use crate::common::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", endian = "little")]
#[repr(u8)]
pub enum STOCMsg {
    #[deku(id = 0x01)]
    GameMsg = 0x01,
    #[deku(id = 0x02)]
    ErrorMsg(ErrorMsg) = 0x02,
    #[deku(id = 0x03)]
    ChooseRPS = 0x03,
    #[deku(id = 0x04)]
    ChooseOrder = 0x04,
    #[deku(id = 0x05)]
    RPSResult(RPSResult) = 0x05,
    #[deku(id = 0x06)]
    OrderResult = 0x06,
    #[deku(id = 0x07)]
    ChangeSide = 0x07,
    #[deku(id = 0x08)]
    WaitingSide = 0x08,
    #[deku(id = 0x11)]
    CreateGame(CreateGame) = 0x11,
    #[deku(id = 0x12)]
    JoinGame(JoinGame) = 0x12,
    #[deku(id = 0x13)]
    TypeChange(TypeChange) = 0x13,
    #[deku(id = 0x14)]
    LeaveGame = 0x14,
    #[deku(id = 0x15)]
    DuelStart = 0x15,
    #[deku(id = 0x16)]
    DuelEnd = 0x16,
    #[deku(id = 0x17)]
    Replay = 0x17,
    #[deku(id = 0x18)]
    TimeLimit(TimeLimit) = 0x18,
    #[deku(id = 0x19)]
    Chat(Chat) = 0x19,
    #[deku(id = 0x20)]
    PlayerEnter(PlayerEnter) = 0x20,
    #[deku(id = 0x21)]
    PlayerChange(PlayerChange) = 0x21,
    #[deku(id = 0x22)]
    WatchChange = 0x22,
    #[deku(id = 0x30)]
    NewReplay = 0x30,
    #[deku(id = 0xF0)]
    Catchup(Catchup) = 0xF0,
    #[deku(id = 0xF1)]
    Rematch = 0xF1,
    #[deku(id = 0xF2)]
    WaitingRematch = 0xF2,
    #[deku(id = 0xF3)]
    Chat2(Chat2) = 0xF3,
}


#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct ErrorMsg {
    pub code: u32,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct RPSResult {
    pub res_0: u8,
    pub res_1: u8,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct CreateGame {
    pub game_id: u32,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct JoinGame {
    pub info: HostInfo,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct TypeChange {
    pub to_type: u8,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct TimeLimit {
    pub team: u8,
    pub left_time: u16,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct PlayerEnter {
    pub name: [u16; 20],
    #[deku(pad_bytes_after = "1")]
    pub pos: u8,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct PlayerChange {
    pub status: u8,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct WatchChange {
    pub count: u16,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct Catchup {
    pub is_catching_up: u16,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct Chat {
    player: u16,
    msg: [u16; 256],
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct Chat2 {
    player_type: u8,
    is_team: u8,
    client_name: [u16; 20],
    msg: [u16; 256],
}


#[cfg(test)]
mod tests {
    use crate::utils;
    use crate::tests::cast_hex_stream_to_c_array;

    use super::*;

    #[test]
    fn stoc_create_game() {
        let packet = "0500110d000000";
        let bytes = cast_hex_stream_to_c_array(packet);
        let (rest, stoc) = HostRequest::<STOCMsg>::from_bytes((&bytes, 0)).unwrap();
        println!("{:?}", stoc);
    }

    #[test]
    fn stoc_stream() {
        let packet = "450012aaec1fd70300000000000000401f00000501b40000000000016201f128010a0001000000010000000100000000e8020000000000000028003c0000000f0000000f0000002b002050006c0061007900650072000000000000000000000000000000000000000000000000000000000000000200210a02001310";
        let bytes = cast_hex_stream_to_c_array(packet);
        let (rest, stoc) = HostRequestStream::<STOCMsg>::from_bytes((&bytes, 0)).unwrap();
    }
}