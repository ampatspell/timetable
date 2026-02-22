use core::ops::Add;

use embedded_graphics::prelude::Point;
use no_std_strings::str32;
use profont::PROFONT_18_POINT;

use crate::{
    Display,
    components::{
        icons::Icons,
        utils::{draw_text, float_to_string},
    },
};

pub struct BlockContext<'a> {
    pub icons: Icons<'a>,
}

pub struct Block {
    icon: str32,
    lines: [str32; 2],
}

impl Block {
    pub fn new() -> Self {
        let icon = str32::new();
        let lines = [str32::new(), str32::new()];
        Self { icon, lines }
    }
    pub fn draw_at(
        &self,
        display: &mut impl Display,
        context: &BlockContext,
        origin: Point,
    ) -> u32 {
        let icon = self.icon;
        let mut y = 0;

        if !icon.is_empty() {
            context
                .icons
                .draw_at(display, self.icon.to_str(), origin.add(Point::new(0, 0)));

            let mut point = origin.add(Point::new(35, -4));

            self.lines
                .iter()
                .filter(|line| line.len() > 0)
                .for_each(|line| {
                    let rect = draw_text(display, point, &line, &PROFONT_18_POINT);
                    let size = rect.size;
                    point = point.add(Point::new(0, size.height as i32));
                    y += size.height + 3;
                });
        }
        y
    }
}

pub struct Blocks<'a> {
    origin: Point,
    blocks: [Block; 6],
    context: BlockContext<'a>,
    value: u32,
}

impl<'a> Blocks<'a> {
    pub fn new(origin: Point) -> Self {
        let icons = Icons::new();
        let blocks = [
            Block::new(),
            Block::new(),
            Block::new(),
            Block::new(),
            Block::new(),
            Block::new(),
        ];
        let context = BlockContext { icons };
        Self {
            origin,
            blocks,
            context,
            value: 0,
        }
    }

    pub fn draw(&self, display: &mut impl Display) {
        let mut y: u32 = 0;
        self.blocks.iter().for_each(|block| {
            let origin = self.origin.add(Point::new(0, y as i32));
            y += block.draw_at(display, &self.context, origin);
        });
    }

    pub fn update(&mut self) {
        self.value += 1;
        self.blocks[0].icon = str32::from("clock");
        self.blocks[0].lines[0] = str32::from(float_to_string(self.value as f32));
    }
}
