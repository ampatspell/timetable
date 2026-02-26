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
            &MAPPING,
        )];

        Self { map }
    }

    pub fn for_size(&self, height: u32) -> Option<&Font<'_>> {
        let font = self.map.iter().find(|font| font.size.height == height);

        font
    }
}

pub struct Font<'a> {
    pub size: Size,
    image: ImageAlpha<'a>,
    mapping: &'a [&'a str],
}

impl<'a> Font<'a> {
    pub fn new(data: &'a [u8], size: Size, mapping: &'a [&'a str]) -> Self {
        let process = {
            static CELL: StaticCell<BlendInBackground> = StaticCell::new();
            CELL.init(BlendInBackground::new())
        };
        let image = ImageAlpha::new(data, size.width as u32, process);

        Self {
            size,
            image,
            mapping,
        }
    }

    pub fn glyph_for_character(&self, c: char) -> Option<u8> {
        if c == ' ' {
            return None;
        }

        let idx = self
            .mapping
            .iter()
            .position(|p| p.chars().next().unwrap() == c)
            .unwrap();

        Some(idx as u8)
    }

    pub fn draw_glyph_at(&self, display: &mut impl Display, glyph: u8, position: Point) -> Point {
        let size = self.size;
        let rect = Rectangle::new(Point::new(0, (size.height * glyph as u32) as i32), size);
        let sub = self.image.sub_image(&rect);
        let image = Image::new(&sub, position);
        image.draw(display).ok();

        position.add(Point::new(size.width as i32, 0))
    }

    pub fn draw_string_at(
        &self,
        display: &mut impl Display,
        string: &str,
        position: Point,
    ) -> Point {
        let mut point = position;
        string.char_indices().for_each(|(_, c)| {
            let glyph = self.glyph_for_character(c);
            point = match glyph {
                Some(glyph) => self.draw_glyph_at(display, glyph, point),
                None => point.add(Point::new(self.size.width as i32, 0)),
            };
        });

        point
    }
}

const MAPPING: [&str; 95] = [
    "×", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f", "g", "h",
    "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "ā",
    "č", "ē", "ģ", "ī", "ķ", "ļ", "ņ", "š", "ū", "ž", "A", "B", "C", "D", "E", "F", "G", "H", "I",
    "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "Ā", "Č",
    "Ē", "Ģ", "Ī", "Ķ", "Ļ", "Ņ", "Š", "Ū", "Ž", "?", "!", "(", ")", ":", ",", ".", "°", "+", "-",
];
