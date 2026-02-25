use core::ops::Add;
use embedded_graphics::prelude::*;
use no_std_strings::str32;

use crate::{
    Display,
    components::{
        blocks::block::{Block, BlockContext},
        fonts::Fonts,
        icons::Icons,
    },
    payload::BlockPayload,
};

pub struct Blocks<'a> {
    origin: Point,
    blocks: [Block; 6],
    context: BlockContext<'a>,
}

impl<'a> Blocks<'a> {
    pub fn new(origin: Point) -> Self {
        let icons = Icons::new();
        let fonts = Fonts::new();
        let context = BlockContext { fonts, icons };
        let blocks = [
            Block::new(),
            Block::new(),
            Block::new(),
            Block::new(),
            Block::new(),
            Block::new(),
        ];
        Self {
            origin,
            blocks,
            context,
        }
    }

    pub fn draw(&mut self, display: &mut impl Display) {
        // let mut y: i32 = 0;
        // let mut force = false;
        // self.blocks.iter_mut().for_each(|block| {
        //     let origin = self.origin.add(Point::new(0, y as i32));
        //     let result = block.draw_at(display, &self.context, origin, false);
        //     y = y + result.height;
        //     if result.needs_layout {
        //         force = true;
        //     }
        // });
        self.context
            .fonts
            .for_size(20)
            .unwrap()
            .draw_at(display, 0, Point::new(20, 20));
    }

    pub fn on_time(&mut self) {
        let block = self.blocks.get_mut(0).unwrap();
        block.update(
            str32::from("clock"),
            [str32::from("01:02:22"), str32::new()],
        )
    }

    pub fn on_data(&mut self, payload: &[BlockPayload; 5]) {
        let mut idx = 0;
        payload.iter().for_each(|payload| {
            let block = self.blocks.get_mut(idx + 1).unwrap();
            let icon = payload.icon;
            let lines = payload.lines;
            block.update(icon, lines);
            idx += 1;
        });
    }
}
