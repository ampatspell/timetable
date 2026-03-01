use crate::{
    Display,
    components::{BACKGROUND_COLOR, blocks::blocks::Blocks},
    payload::BlockPayload,
};
use embedded_graphics::prelude::*;
use no_std_strings::str12;

pub struct UI<'a> {
    blocks: Blocks<'a>,
}

impl<'a> UI<'a> {
    pub fn new() -> Self {
        let blocks = Blocks::new(Point::new(45, 20));
        Self { blocks }
    }

    pub fn prepare(&mut self, display: &mut impl Display) {
        display.clear(BACKGROUND_COLOR).ok();
        self.blocks.on_start();
        self.draw(display);
    }

    fn draw(&mut self, display: &mut impl Display) -> () {
        self.blocks.draw(display);
    }

    pub fn on_message(&mut self, display: &mut impl Display, block: BlockPayload) {
        self.blocks.on_message(&block);
        self.draw(display);
    }

    pub fn on_weather(&mut self, display: &mut impl Display, blocks: [BlockPayload; 4]) {
        self.blocks.on_weather(&blocks);
        self.draw(display);
    }

    pub fn on_timetable(&mut self, display: &mut impl Display, block: BlockPayload) {
        self.blocks.on_timetable(&block);
        self.draw(display);
    }

    pub fn on_time(&mut self, display: &mut impl Display, string: str12) {
        self.blocks.on_time(&string);
        self.draw(display);
    }
}
