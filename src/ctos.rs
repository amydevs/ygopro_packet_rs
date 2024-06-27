use deku::prelude::*;
use crate::common::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", endian = "little")]
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
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct PlayerInfo {
    pub name: [u16; 20],
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct CreateGame {
    pub info: HostInfo,
    pub name: [u16; 20],
    pub password: [u16; 20],
    pub notes: [u8; 200],
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct JoinGame {
    pub version: u16,
    pub game_id: u32,
    pub password: [u16; 20],
    pub version_2: ClientVersion,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct Kick {
    pub pos: u8
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct Rematch {
    pub rematch: u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let packet = "29001050006c00610079006500720000006aa1ee550000000000000c7f000030dfffffffffffff00508fee5d0111aaec1fd70300000000550000401f00000501b40000000000016201f128010a0001000000010000000100000000e8020000000000000028003c0000000f0000000f00000049003a0050000000151c5f9eee5500005029008c0c7f000008bb7fa70c7f000018bb7fa70c7f0000310032003300000000000000000000002027008c0c7f000000000000000000000000000000000000313233000c7f0000000000000000000050b2028c0c7f00000000000000000000000000000000000098c46aa1ee5500000000000000000000c8bd7fa70c7f00009fba7fa70c7f0000f0c36aa1ee550000f896e5a1ee550000f063c09eee5500002027008c0c7f0000b0bfc09eee5500005029008c0c7f000010d2bf9eee5500003068008c0c7f0000705fbe9eee55000050b2028c0c7f0000000000000000000000000000000000000100000001000000000000000000000000000000000000000000000000000000";
        let bytes: Vec<u8>  = (0..packet.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&packet[i..i + 2], 16))
            .collect::<Result<Vec<_>, _>>().unwrap();
        let (rest, ctos) = HostRequest::<CTOSMsg>::from_bytes((&bytes, 0)).unwrap();
        // if let CTOSMsg::PlayerInfo(thing) = ctos.body  {
        //     println!("{:?}", String::from_utf16_lossy(&thing.name));
        // }
        let (rest, ctos) = HostRequest::<CTOSMsg>::from_bytes(rest).unwrap();
        // if let CTOSMsg::CreateGame(thing) = ctos.body  {
        //     println!("{:?}", String::from_utf8(thing.notes.to_vec()));
        // }
        println!("{:?}", rest.0);
    }
}

