use no_std_strings::{str8, str16};

#[derive(Copy, Clone)]
pub struct BlockPayload {
    pub index: u8,
    pub icon: str8,
    pub lines: [str16; 2],
}

#[derive(Copy, Clone)]
pub struct Payload {
    pub blocks: [BlockPayload; 6],
}
