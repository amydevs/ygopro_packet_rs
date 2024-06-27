use deku::{ctx::Endian, prelude::*};

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct HostRequest<T: DekuWriter + for<'a> DekuReader<'a> > {
    #[deku(endian = "big")]
    pub id: u16,
    pub body: T
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct HostRequestStream<T: DekuWriter + for<'a> DekuReader<'a>> {
    #[deku(read_all)]
    pub requests: Vec<HostRequest<T>>
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct HostInfo {
    pub lflist: u32,
    pub rule: u8,
    pub mode: u8,
    pub duel_rule: u8,
    pub no_check_deck_content: u8,
    #[deku(pad_bytes_after = "3")]
    pub no_shuffle_deck: u8,
    pub start_lp: u32,
    pub start_hand: u8,
    pub draw_count: u8,
    pub time_limit: u16,
    pub duel_flag_high: u32,
    pub handshake: u32,
    pub version: ClientVersion,
    pub team1: i32,
    pub team2: i32,
    pub best_of: i32,
    pub duel_flags_low: u32,
    pub forbidden_types: u32,
    pub extra_rules: u16,
    #[deku(pad_bytes_after = "2")]
    pub sizes: DeckSizes,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct ClientVersion {
    pub client: Version,
    pub core: Version,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct Sizes {
    pub min: u16,
    pub max: u16,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct DeckSizes {
    pub main: Sizes,
    pub extra: Sizes,
    pub side: Sizes,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", endian = "endian", ctx = "endian: deku::ctx::Endian")]
#[repr(u8)]
pub enum PlayerType {
    #[deku(id = 0x00)]
    Player = 0x00,
    #[deku(id = 0x01)]
    Observer = 0x01,
    #[deku(id = 0x02)]
    System = 0x02,
    #[deku(id = 0x03)]
    SystemError = 0x03,
    #[deku(id = 0x04)]
    SystemShout = 0x04,
}