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

pub fn parse(body: &str) -> Payload {
    let mut iter = body.split('\n').into_iter();

    let year = iter.next().unwrap().parse::<u16>();
    let month = iter.next().unwrap().parse::<u16>();
    let date = iter.next().unwrap().parse::<u16>();
    let h = iter.next().unwrap().parse::<u16>();
    let m = iter.next().unwrap().parse::<u16>();
    let s = iter.next().unwrap().parse::<u16>();

    let mut blocks = [BlockPayload {
        index: 0,
        icon: str32::new(),
        lines: [str32::new(), str32::new()],
    }; 6];

    for index in 0..5 {
        let mut block = blocks[index];
        block.icon = str32::from(iter.next().unwrap());
        block.lines = [
            str32::from(iter.next().unwrap()),
            str32::from(iter.next().unwrap()),
        ];
        blocks[index] = block;
    }

    Payload { blocks }
}
