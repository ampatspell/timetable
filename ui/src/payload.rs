use no_std_strings::{str32, str128};

#[derive(Copy, Clone)]
pub struct LinePayload {
    pub value: str128,
}

#[derive(Copy, Clone)]
pub struct BlockPayload {
    pub index: u8,
    pub icon: str32,
    pub lines: [LinePayload; 2],
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
        lines: [
            LinePayload {
                value: str128::new(),
            },
            LinePayload {
                value: str128::new(),
            },
        ],
    }; 6];

    for index in 0..5 {
        let mut block = blocks[index];
        block.icon = str32::from(iter.next().unwrap());
        block.lines[0].value = str128::from(iter.next().unwrap());
        block.lines[1].value = str128::from(iter.next().unwrap());
        blocks[index] = block;
    }

    Payload { blocks }
}
