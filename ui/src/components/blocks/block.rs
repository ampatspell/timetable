use core::ops::Add;

use embedded_graphics::prelude::Point;
use no_std_strings::{str12, str32};

use crate::{
    Display,
    components::{fonts::Fonts, icons::Icons},
};

pub struct BlockContext<'a> {
    pub icons: Icons<'a>,
    pub fonts: Fonts<'a>,
}

pub struct BlockDrawResult {
    pub height: i32,
    pub needs_layout: bool,
}

pub struct Block {
    icon: str12,
    lines: [str32; 2],
}

impl Block {
    pub fn new() -> Self {
        let icon = str12::new();
        let lines = [str32::new(), str32::new()];
        Self { icon, lines }
    }

    pub fn draw_at(
        &mut self,
        display: &mut impl Display,
        context: &BlockContext,
        origin: Point,
    ) -> i32 {
        let icon = self.icon;
        let mut u_height: u32 = 0;

        if !icon.is_empty() {
            context
                .icons
                .draw_at(display, self.icon.to_str(), origin.add(Point::new(0, 0)));

            let mut point = origin.add(Point::new(35, 0));
            let font = context.fonts.for_size(20).unwrap();

            self.lines
                .iter()
                .filter(|line| line.len() > 0)
                .for_each(|line| {
                    font.draw_string_at_clear(display, &line, point, 26);
                    point = point.add(Point::new(0, font.size.height as i32));
                    u_height += font.size.height;
                });
        }

        let height = u_height as i32;

        height
    }

    pub fn update(&mut self, icon: str12, lines: [str32; 2]) {
        self.icon = icon;
        self.lines = lines;
    }
}
