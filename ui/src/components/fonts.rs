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
            10,
            20,
        )];
        Self { map }
    }

    pub fn for_size(&self, height: u8) -> Option<&Font<'_>> {
        let font = self.map.iter().find(|font| font.height == height);

        font
    }
}

static GLYPHS: u8 = 85;

pub struct Font<'a> {
    width: u8,
    height: u8,
    image: ImageAlpha<'a>,
}

impl<'a> Font<'a> {
    pub fn new(data: &'a [u8], width: u8, height: u8) -> Self {
        let process = {
            static CELL: StaticCell<BlendInBackground> = StaticCell::new();
            CELL.init(BlendInBackground::new())
        };
        let image = ImageAlpha::new(data, width as u32, process);
        Self {
            width,
            height,
            image,
        }
    }

    pub fn draw_at(&self, display: &mut impl Display, glyph: u16, position: Point) {
        let rect = Rectangle::new(Point::new(0, 0), Size::new(10, 20));
        let sub = self.image.sub_image(&rect);
        let image = Image::new(&sub, position);
        let result = image.draw(display);
    }
}
