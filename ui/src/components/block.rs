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
    pub fn new(icon: &'a str, lines: [str32; 2]) -> Self {
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

        let mut point = origin.add(Point::new(30, -4));
        let mut y = 0;

        self.lines
            .iter()
            .filter(|line| line.len() > 0)
            .for_each(|line| {
                let rect = draw_text(display, point, &line, &PROFONT_18_POINT);
                let size = rect.size;
                point = point.add(Point::new(0, size.height as i32));
                y += 24;
            });

        y
    }
}

pub struct Blocks<'a> {
    origin: Point,
    blocks: [Block<'a>; 6],
    context: BlockContext<'a>,
}

impl<'a> Blocks<'a> {
    pub fn new(origin: Point, icons: &'a Icons<'a>) -> Self {
        let blocks = [
            Block::new("clock", [str32::from("01:04:42"), str32::new()]),
            Block::new(
                "cloud-snow",
                [str32::from("-05.70"), str32::from("Snow grains falling.")],
            ),
            Block::new("sun", [str32::from("01"), str32::new()]),
            Block::new("sunrise", [str32::from("06:39:10"), str32::new()]),
            Block::new("sunset", [str32::from("03:11:45"), str32::new()]),
            Block::new(
                "bus-stop",
                [str32::from("01:12:00 -02m"), str32::from("01:28:00 +30s")],
            ),
        ];
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
