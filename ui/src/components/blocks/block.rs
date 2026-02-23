use core::ops::Add;

use embedded_graphics::prelude::Point;
use no_std_strings::str32;
use profont::PROFONT_18_POINT;

use crate::{
    Display,
    components::{icons::Icons, utils::draw_text},
};

pub struct BlockContext<'a> {
    pub icons: Icons<'a>,
}

pub struct BlockDrawResult {
    pub height: i32,
    pub needs_layout: bool,
}

pub struct Block {
    icon: str32,
    lines: [str32; 2],
    needs_draw: bool,
    height: i32,
}

impl Block {
    pub fn new() -> Self {
        let icon = str32::new();
        let lines = [str32::new(), str32::new()];
        Self {
            icon,
            lines,
            needs_draw: true,
            height: 0,
        }
    }

    pub fn draw_at(
        &mut self,
        display: &mut impl Display,
        context: &BlockContext,
        origin: Point,
        force: bool,
    ) -> BlockDrawResult {
        if self.needs_draw || force {
            let icon = self.icon;
            let mut u_height: u32 = 0;

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
                        u_height += size.height + 3;
                    });
            }

            let height = u_height as i32;
            let needs_layout = self.height != height;
            self.height = height;
            self.needs_draw = false;

            return BlockDrawResult {
                height: height,
                needs_layout,
            };
        }

        BlockDrawResult {
            height: self.height,
            needs_layout: false,
        }
    }

    pub fn set_needs_draw(&mut self) {
        self.needs_draw = true;
    }

    pub fn update(&mut self, icon: str32, lines: [str32; 2]) {
        if !self.icon.eq(&icon) {
            self.icon = icon;
            self.set_needs_draw();
        }

        if !self.lines[0].eq(&lines[0]) {
            self.lines[0] = lines[0];
            self.set_needs_draw();
        }

        if !self.lines[1].eq(&lines[1]) {
            self.lines[1] = lines[1];
            self.set_needs_draw();
        }
    }
}
