use deku::prelude::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct HostRequest<T: DekuWriter + for<'a> DekuReader<'a> > {
    #[deku(endian = "little")]
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
    pub team_1: i32,
    pub team_2: i32,
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
