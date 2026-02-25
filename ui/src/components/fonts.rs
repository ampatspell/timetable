use core::{iter::Map, ops::Add, str::CharIndices};

use embedded_graphics::{image::Image, prelude::*, primitives::Rectangle};
use no_std_strings::str32;
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
        let mapping = {
            static CELL: StaticCell<&[(&str, u8)]> = StaticCell::new();
            CELL.init(&[
                (" ", 0),
                ("×", 1),
                ("0", 2),
                ("1", 3),
                ("2", 4),
                ("3", 5),
                ("4", 6),
                ("5", 7),
                ("6", 8),
                ("7", 9),
                ("8", 10),
                ("9", 11),
                ("a", 12),
                ("b", 13),
                ("c", 14),
                ("d", 15),
                ("e", 16),
                ("f", 17),
                ("g", 18),
                ("h", 19),
                ("i", 20),
                ("j", 21),
                ("k", 22),
                ("l", 23),
                ("m", 24),
                ("n", 25),
                ("o", 26),
                ("p", 27),
                ("q", 28),
                ("r", 29),
                ("s", 30),
                ("t", 31),
                ("u", 32),
                ("v", 33),
                ("w", 34),
                ("x", 35),
                ("y", 36),
                ("z", 37),
                ("ā", 38),
                ("č", 39),
                ("ē", 40),
                ("ģ", 41),
                ("ī", 42),
                ("ķ", 43),
                ("ļ", 44),
                ("ņ", 45),
                ("š", 46),
                ("ū", 47),
                ("ž", 48),
                ("A", 49),
                ("B", 50),
                ("C", 51),
                ("D", 52),
                ("E", 53),
                ("F", 54),
                ("G", 55),
                ("H", 56),
                ("I", 57),
                ("J", 58),
                ("K", 59),
                ("L", 60),
                ("M", 61),
                ("N", 62),
                ("O", 63),
                ("P", 64),
                ("Q", 65),
                ("R", 66),
                ("S", 67),
                ("T", 68),
                ("U", 69),
                ("V", 70),
                ("W", 71),
                ("X", 72),
                ("Y", 73),
                ("Z", 74),
                ("Ā", 75),
                ("Č", 76),
                ("Ē", 77),
                ("Ģ", 78),
                ("Ī", 79),
                ("Ķ", 80),
                ("Ļ", 81),
                ("Ņ", 82),
                ("Š", 83),
                ("Ū", 84),
                ("Ž", 85),
                ("°", 86),
                ("+", 87),
                ("-", 88),
            ])
        };

        let map = [Font::new(
            include_bytes!("../../../font/out/font-10x20.raw"),
            Size::new(10, 20),
            mapping,
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
    mapping: &'a [(&'a str, u8)],
}

impl<'a> Font<'a> {
    pub fn new(data: &'a [u8], size: Size, mapping: &'a [(&'a str, u8)]) -> Self {
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

    fn map_characters_to_indices(&self, string: &str) -> [u8; 32] {
        let indices = string.char_indices().map(|(_, string_char)| {
            let mut string_buffer = [0; 2];
            string_char.encode_utf8(&mut string_buffer);
            let mapped = self.mapping.iter().find(|(s, _)| {
                let first_char = s.chars().next().unwrap();
                let mut mapping_buffer = [0; 2];
                first_char.encode_utf8(&mut mapping_buffer);
                return string_buffer[0] == mapping_buffer[0]
                    && string_buffer[1] == mapping_buffer[1];
            });
            let res = match mapped {
                Some(f) => f.1,
                None => 0,
            };

            res
        });

        let mut buffer: [u8; 32] = [0; 32];
        let mut idx = 0;
        indices.for_each(|i| {
            buffer[idx] = i;
            idx += 1;
        });

        buffer
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
        let chars = self.map_characters_to_indices(&string);
        let mut point = position;
        chars
            .iter()
            .for_each(|index| point = self.draw_glyph_at(display, *index, point));

        point
    }
}
