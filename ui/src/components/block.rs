use core::ops::Add;

use embedded_graphics::prelude::Point;
use no_std_strings::str32;
use profont::PROFONT_18_POINT;

use crate::{
    Display,
    components::{icons::Icons, utils::draw_text},
};

pub struct BlockContext<'a> {
    pub icons: &'a Icons<'a>,
}

pub struct Block<'a> {
    icon: &'a str,
    lines: [str32; 2],
}

impl<'a> Block<'a> {
    pub fn new(icon: &'a str) -> Self {
        let lines = [str32::from("Hello"), str32::new()];
        Self { icon, lines }
    }
    pub fn draw_at(
        &self,
        display: &mut impl Display,
        context: &BlockContext<'a>,
        origin: Point,
    ) -> i32 {
        context
            .icons
            .draw_at(display, self.icon, origin.add(Point::new(0, 0)));

        let _ = draw_text(
            display,
            origin.add(Point::new(30, -4)),
            &self.lines[0],
            &PROFONT_18_POINT,
        );

        24
    }
}

pub struct Blocks<'a> {
    origin: Point,
    blocks: [Block<'a>; 2],
    context: BlockContext<'a>,
}

impl<'a> Blocks<'a> {
    pub fn new(origin: Point, icons: &'a Icons<'a>) -> Self {
        let blocks = [Block::new("sun"), Block::new("sun")];
        let context = BlockContext { icons };
        Self {
            origin,
            blocks,
            context,
        }
    }

    pub fn draw(&self, display: &mut impl Display) {
        let mut y = 0;
        self.blocks.iter().for_each(|block| {
            let origin = self.origin.add(Point::new(0, y));
            y += block.draw_at(display, &self.context, origin);
        });
    }
}
