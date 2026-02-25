use core::ops::Add;

use embedded_graphics::{image::Image, prelude::*, primitives::Rectangle};
use static_cell::StaticCell;

use crate::{
    Display,
    components::alpha::{BlendInBackground, ImageAlpha},
};

pub struct Fonts<'a> {
    map: [Font<'a>; 1],
}

impl<'a> Fonts<'a> {
    pub fn new() -> Self {
        let map = [Font::new(
            include_bytes!("../../../font/out/font-10x20.raw"),
            Size::new(10, 20),
        )];

        Self { map }
    }

    pub fn for_size(&self, height: u32) -> Option<&Font<'_>> {
        let font = self.map.iter().find(|font| font.size.height == height);

        font
    }
}

pub struct Font<'a> {
    size: Size,
    image: ImageAlpha<'a>,
}

impl<'a> Font<'a> {
    pub fn new(data: &'a [u8], size: Size) -> Self {
        let process = {
            static CELL: StaticCell<BlendInBackground> = StaticCell::new();
            CELL.init(BlendInBackground::new())
        };
        let image = ImageAlpha::new(data, size.width as u32, process);

        Self { size, image }
    }

    pub fn draw_glyph_at(&self, display: &mut impl Display, glyph: u32, position: Point) -> Point {
        let size = self.size;
        let rect = Rectangle::new(Point::new(0, (size.height * glyph) as i32), size);
        let sub = self.image.sub_image(&rect);
        let image = Image::new(&sub, position);
        image.draw(display).ok();

        position.add(Point::new(size.width as i32, 0))
    }
}
