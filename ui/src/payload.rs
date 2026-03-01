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

pub enum Visual {
    Time { time: str12 },
    Weather { blocks: [BlockPayload; 4] },
    Timetable { block: BlockPayload },
    Message { message: BlockPayload },
}
