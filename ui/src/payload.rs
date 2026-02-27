use no_std_strings::{str12, str32};

#[derive(Copy, Clone)]
pub struct BlockPayload {
    pub icon: str12,
    pub lines: [str32; 2],
}

#[derive(Copy, Clone)]
pub struct Payload {
    pub blocks: [BlockPayload; 6],
}
