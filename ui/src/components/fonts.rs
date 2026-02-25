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
        let mapping = {
            static CELL: StaticCell<&[(&str, u8)]> = StaticCell::new();
            CELL.init(&[
                ("×", 0),
                ("0", 1),
                ("1", 2),
                ("2", 3),
                ("3", 4),
                ("4", 5),
                ("5", 6),
                ("6", 7),
                ("7", 8),
                ("8", 9),
                ("9", 10),
                ("a", 11),
                ("b", 12),
                ("c", 13),
                ("d", 14),
                ("e", 15),
                ("f", 16),
                ("g", 17),
                ("h", 18),
                ("i", 19),
                ("j", 20),
                ("k", 21),
                ("l", 22),
                ("m", 23),
                ("n", 24),
                ("o", 25),
                ("p", 26),
                ("q", 27),
                ("r", 28),
                ("s", 29),
                ("t", 30),
                ("u", 31),
                ("v", 32),
                ("w", 33),
                ("x", 34),
                ("y", 35),
                ("z", 36),
                ("ā", 37),
                ("č", 38),
                ("ē", 39),
                ("ģ", 40),
                ("ī", 41),
                ("ķ", 42),
                ("ļ", 43),
                ("ņ", 44),
                ("š", 45),
                ("ū", 46),
                ("ž", 47),
                ("A", 48),
                ("B", 49),
                ("C", 50),
                ("D", 51),
                ("E", 52),
                ("F", 53),
                ("G", 54),
                ("H", 55),
                ("I", 56),
                ("J", 57),
                ("K", 58),
                ("L", 59),
                ("M", 60),
                ("N", 61),
                ("O", 62),
                ("P", 63),
                ("Q", 64),
                ("R", 65),
                ("S", 66),
                ("T", 67),
                ("U", 68),
                ("V", 69),
                ("W", 70),
                ("X", 71),
                ("Y", 72),
                ("Z", 73),
                ("Ā", 74),
                ("Č", 75),
                ("Ē", 76),
                ("Ģ", 77),
                ("Ī", 78),
                ("Ķ", 79),
                ("Ļ", 80),
                ("Ņ", 81),
                ("Š", 82),
                ("Ū", 83),
                ("Ž", 84),
                ("?", 85),
                ("!", 86),
                ("(", 87),
                (")", 88),
                (":", 89),
                (",", 90),
                (".", 91),
                ("°", 92),
                ("+", 93),
                ("-", 94),
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
    pub size: Size,
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
