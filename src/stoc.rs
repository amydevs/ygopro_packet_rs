use deku::prelude::*;
use crate::common::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", endian = "little")]
#[repr(u8)]
pub enum STOCMsg {
    #[deku(id = 0x01)]
    GameMsg = 0x01,
    #[deku(id = 0x02)]
    ErrorMsg(ErrorMsgBody) = 0x02,
    #[deku(id = 0x03)]
    ChooseRPS = 0x03,
    #[deku(id = 0x04)]
    ChooseOrder = 0x04,
    #[deku(id = 0x05)]
    RPSResult(RPSResultBody) = 0x05,
    #[deku(id = 0x06)]
    OrderResult = 0x06,
    #[deku(id = 0x07)]
    ChangeSide = 0x07,
    #[deku(id = 0x08)]
    WaitingSide = 0x08,
    #[deku(id = 0x11)]
    CreateGame(CreateGameBody) = 0x11,
    #[deku(id = 0x12)]
    JoinGame(JoinGameBody) = 0x12,
    #[deku(id = 0x13)]
    TypeChange(TypeChangeBody) = 0x13,
    #[deku(id = 0x14)]
    LeaveGame = 0x14,
    #[deku(id = 0x15)]
    DuelStart = 0x15,
    #[deku(id = 0x16)]
    DuelEnd = 0x16,
    #[deku(id = 0x17)]
    Replay = 0x17,
    #[deku(id = 0x18)]
    TimeLimit(TimeLimitBody) = 0x18,
    #[deku(id = 0x19)]
    Chat(ChatBody) = 0x19,
    #[deku(id = 0x20)]
    PlayerEnter(PlayerEnterBody) = 0x20,
    #[deku(id = 0x21)]
    PlayerChange(PlayerChangeBody) = 0x21,
    #[deku(id = 0x22)]
    WatchChange = 0x22,
    #[deku(id = 0x30)]
    NewReplay = 0x30,
    #[deku(id = 0xF0)]
    Catchup(CatchupBody) = 0xF0,
    #[deku(id = 0xF1)]
    Rematch = 0xF1,
    #[deku(id = 0xF2)]
    WaitingRematch = 0xF2,
    #[deku(id = 0xF3)]
    Chat2(Chat2Body) = 0xF3,
}


#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", endian = "endian", ctx = "endian: deku::ctx::Endian")]
#[repr(u8)]
pub enum ErrorMsgBody {
    #[deku(id = 0x01)]
    JoinError(
        #[deku(pad_bytes_before = "3")]
        JoinErrorType
    ) = 0x01,
    #[deku(id = 0x02)]
    DeckError(
        #[deku(pad_bytes_before = "3")]
        DeckErrrorDetails
    ) = 0x02,
    #[deku(id = 0x03)]
    SideError(
        #[deku(pad_bytes_before = "3")]
        u32
    ) = 0x03,
    #[deku(id = 0x04)]
    VersionError(
        #[deku(pad_bytes_before = "3")]
        u32
    ) = 0x04,
    #[deku(id = 0x05)]
    VersionError2(
        #[deku(pad_bytes_before = "3")]
        ClientVersion
    ) = 0x05,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u32", endian = "endian", ctx = "endian: deku::ctx::Endian")]
#[repr(u32)]
pub enum JoinErrorType {
    Unable = 0x00,
    Password,
    Refused
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct DeckErrrorDetails {
    pub error: DeckErrorType,
    pub count: DeckErrorCount,
    pub code: u32,
}


#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u32", endian = "endian", ctx = "endian: deku::ctx::Endian")]
#[repr(u32)]
pub enum DeckErrorType {
    None = 0x00,
    Lflist,
    OCGOnly,
    TCGOnly,
    UnknownCard,
    CardCount,
    MainCount,
    ExtraCount,
    SideCount,
    ForbType,
    UnofficialCard,
    InvalidSize,
    TooManyLegends,
    TooManySkills,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct DeckErrorCount {
    pub got: u32,
    pub min: u32,
    pub max: u32,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct RPSResultBody {
    pub res_0: u8,
    pub res_1: u8,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct CreateGameBody {
    pub game_id: u32,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct JoinGameBody {
    pub info: HostInfo,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct TypeChangeBody {
    pub to_type: u8,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct TimeLimitBody {
    pub team: u8,
    pub left_time: u16,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct PlayerEnterBody {
    pub name: [u16; 20],
    #[deku(pad_bytes_after = "1")]
    pub pos: u8,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct PlayerChangeBody {
    pub status: u8,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct WatchChangeBody {
    pub count: u16,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct CatchupBody {
    pub is_catching_up: u16,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct ChatBody {
    player: u16,
    msg: [u16; 256],
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct Chat2Body {
    player_type: Chat2PlayerType,
    is_team: u8,
    client_name: [u16; 20],
    msg: [u16; 256],
}


#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", endian = "endian", ctx = "endian: deku::ctx::Endian")]
#[repr(u8)]
pub enum Chat2PlayerType {
    Player = 0x00,
    Observer,
    System,
    SystemError,
    SystemShout,
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