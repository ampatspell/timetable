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

#[derive(defmt::Format)]
pub struct TimeData {
    pub year: u16,
    pub month: u16,
    pub date: u16,
    pub hours: u16,
    pub minutes: u16,
    pub seconds: u16,
    pub millis: u16,
}

impl TimeData {
    pub fn parse(body: &str) -> Self {
        let mut iter = body.split('\n').into_iter();
        let mut parse = || iter.next().unwrap().parse::<u16>().unwrap();
        Self {
            year: parse(),
            month: parse(),
            date: parse(),
            hours: parse(),
            minutes: parse(),
            seconds: parse(),
            millis: parse(),
        }
    }
}
