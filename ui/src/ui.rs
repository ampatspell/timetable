use crate::{
    Display,
    components::{BACKGROUND_COLOR, blocks::blocks::Blocks},
    payload::BlockPayload,
};
use embedded_graphics::prelude::*;
use no_std_strings::{str8, str12, str16};

pub struct UI<'a> {
    blocks: Blocks<'a>,
}

impl<'a> UI<'a> {
    pub fn new() -> Self {
        let blocks = Blocks::new(Point::new(35, 25));
        Self { blocks }
    }

    pub fn prepare(&mut self, display: &mut impl Display) {
        display.clear(BACKGROUND_COLOR).ok();
    }

    pub fn draw(&mut self, display: &mut impl Display) -> () {
        self.blocks.draw(display);
    }

    pub fn update(&mut self) {
        let blocks = [
            BlockPayload {
                index: 0,
                icon: str8::from("cloud-snow"),
                lines: [str16::from("-05.70°"), str16::from("Snow grains fall")],
            },
            BlockPayload {
                index: 1,
                icon: str8::from("sun"),
                lines: [str16::from("01"), str16::new()],
            },
            BlockPayload {
                index: 2,
                icon: str8::from("sunrise"),
                lines: [str16::from("06:39:10"), str16::new()],
            },
            BlockPayload {
                index: 3,
                icon: str8::from("sunset"),
                lines: [str16::from("03:11:45"), str16::new()],
            },
            BlockPayload {
                index: 4,
                icon: str8::from("bus-stop"),
                lines: [str16::from("01:12:00 -02m"), str16::from("01:28:00 +30s")],
            },
        ];
        self.blocks.on_data(&blocks);
    }

    pub fn on_time(&mut self, display: &mut impl Display, string: str12) {
        self.blocks.on_time(&string);
        self.draw(display);
    }
}
