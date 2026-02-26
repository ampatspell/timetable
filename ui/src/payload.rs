use no_std_strings::str32;

#[derive(Copy, Clone)]
pub struct BlockPayload {
    pub index: u8,
    pub icon: str32,
    pub lines: [str32; 2],
}

#[derive(Copy, Clone)]
pub struct Payload {
    pub blocks: [BlockPayload; 6],
}
