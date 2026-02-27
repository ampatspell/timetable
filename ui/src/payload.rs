use no_std_strings::{str12, str16};

#[derive(Copy, Clone)]
pub struct BlockPayload {
    pub icon: str12,
    pub lines: [str16; 2],
}

#[derive(Copy, Clone)]
pub struct Payload {
    pub blocks: [BlockPayload; 6],
}
