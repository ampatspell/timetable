use crate::{
    Display,
    components::{BACKGROUND_COLOR, blocks::blocks::Blocks},
    payload::BlockPayload,
};
use embedded_graphics::prelude::*;
use no_std_strings::str32;

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
        self.blocks.on_time();
        let blocks = [
            BlockPayload {
                index: 0,
                icon: str32::from("cloud-snow"),
                lines: [str32::from("-05.70"), str32::from("Snow grains fall")],
            },
            BlockPayload {
                index: 1,
                icon: str32::from("sun"),
                lines: [str32::from("01"), str32::new()],
            },
            BlockPayload {
                index: 2,
                icon: str32::from("sunrise"),
                lines: [str32::from("06:39:10"), str32::new()],
            },
            BlockPayload {
                index: 3,
                icon: str32::from("sunset"),
                lines: [str32::from("03:11:45"), str32::new()],
            },
            BlockPayload {
                index: 4,
                icon: str32::from("bus-stop"),
                lines: [str32::from("01:12:00 -02m"), str32::from("01:28:00 +30s")],
            },
        ];
        self.blocks.on_data(&blocks);
    }
}
