use crate::{
    Display,
    components::{BACKGROUND_COLOR, block::Blocks},
};
use embedded_graphics::prelude::*;

pub struct UI<'a> {
    blocks: Blocks<'a>,
}

impl<'a> UI<'a> {
    pub fn new() -> Self {
        let blocks = Blocks::new(Point::new(35, 25));
        Self { blocks }
    }

    pub fn draw(&self, display: &mut impl Display) -> () {
        self.blocks.draw(display);
    }

    pub fn prepare(&mut self, display: &mut impl Display) {
        display.clear(BACKGROUND_COLOR).ok();
    }

    pub fn update(&mut self) {
        self.blocks.update();
    }
}
