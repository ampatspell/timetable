use embedded_graphics::{
    image::{Image, ImageRaw},
    pixelcolor::{Rgb565, raw::LittleEndian},
    prelude::Point,
};
use no_std_strings::str32;

pub struct Block<'a> {
    icon: &'a ImageRaw<'a, Rgb565, LittleEndian>,
    lines: [str32; 2],
}

impl<'a> Block<'a> {
    pub fn draw_at(&self, origin: Point) -> () {
        let icon = Image::new(self.icon, origin);
        let lines = self.lines;
    }
}
