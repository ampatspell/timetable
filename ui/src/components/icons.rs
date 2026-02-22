use defmt::info;
use embedded_graphics::{
    image::{Image, ImageRaw, ImageRawLE},
    pixelcolor::{Rgb565, raw::LittleEndian},
    prelude::*,
};

use crate::{Display, components::transparent::ImageTransparent};

#[derive(Debug)]
pub struct Icons<'a> {
    map: [Icon<'a>; 1],
}

impl<'a> Icons<'a> {
    pub fn new() -> Self {
        let map = [Icon::new(
            "sun",
            include_bytes!("../../../images/assets/tabler-icon-sun.raw"),
            24,
        )];
        Self { map }
    }

    pub fn draw_at(&self, display: &mut impl Display, name: &str, position: Point) -> () {
        let icon = self.map.iter().find(|icon| icon.name.eq(name));
        match icon {
            Some(icon) => icon.draw_at(display, position),
            None => info!("Icon {} was not found", name),
        }
    }
}

#[derive(Debug)]
pub struct Icon<'a> {
    pub name: &'a str,
    image: ImageTransparent<ImageRaw<'a, Rgb565, LittleEndian>>,
}

impl<'a> Icon<'a> {
    pub fn new(name: &'a str, data: &'a [u8], width: u32) -> Self {
        let raw = ImageRawLE::<Rgb565>::new(data, width);
        let image = ImageTransparent::new(raw, Rgb565::BLACK);

        Self { name, image }
    }

    pub fn draw_at(&self, display: &mut impl Display, position: Point) -> () {
        let image = Image::new(&self.image, position);
        let result = image.draw(display);
        result.ok();
    }
}
