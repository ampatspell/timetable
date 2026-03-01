use crate::{
    Display,
    components::{BACKGROUND_COLOR, blocks::blocks::Blocks},
    payload::Visual,
};
use embedded_graphics::{
    framebuffer::{Framebuffer, buffer_size},
    pixelcolor::{
        Rgb565,
        raw::{LittleEndian, RawU16},
    },
    prelude::*,
};

pub struct UI<'a> {
    blocks: Blocks<'a>,
    framebuffer:
        Framebuffer<Rgb565, RawU16, LittleEndian, 240, 280, { buffer_size::<Rgb565>(240, 280) }>,
}

impl<'a> UI<'a> {
    pub fn new() -> Self {
        let blocks = Blocks::new(Point::new(40, 20));
        let framebuffer = Framebuffer::<
            Rgb565,
            _,
            LittleEndian,
            240,
            280,
            { buffer_size::<Rgb565>(240, 280) },
        >::new();

        Self {
            blocks,
            framebuffer,
        }
    }

    pub fn prepare(&mut self, display: &mut impl Display) {
        self.clear();
        self.blocks.on_start();
        self.draw(display);
    }

    fn clear(&mut self) {
        let framebuffer = &mut self.framebuffer;
        let size = framebuffer.size();
        for x in 0..size.width {
            for y in 0..size.height {
                framebuffer.set_pixel(Point::new(x as i32, y as i32), BACKGROUND_COLOR);
            }
        }
    }

    fn draw(&self, display: &mut impl Display) {
        self.framebuffer.as_image().draw(display).ok();
    }

    pub fn on_message(&mut self, display: &mut impl Display, message: Visual) {
        match message {
            Visual::Time { time } => {
                self.blocks.on_time(&time);
            }
            Visual::Weather { blocks } => {
                self.blocks.on_weather(&blocks);
            }
            Visual::Timetable { block } => {
                self.blocks.on_timetable(&block);
            }
            Visual::Message { message } => {
                self.blocks.on_message(&message);
            }
        };

        self.clear();
        self.blocks.draw(&mut self.framebuffer);
        self.draw(display);
    }
}
