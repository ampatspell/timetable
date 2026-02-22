use crate::{
    Display,
    components::{BACKGROUND_COLOR, TEXT_COLOR},
};
use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyleBuilder},
    pixelcolor::{Rgb565, Rgb888},
    prelude::*,
    primitives::{Line, PrimitiveStyleBuilder, Rectangle},
    text::Text,
};
use no_std_strings::str32;

pub fn float_to_string(value: f32) -> str32 {
    let mut buffer = ryu::Buffer::new();
    let value = buffer.format(value);
    str32::from(value)
}

pub fn draw_text<'a>(
    display: &mut impl Display,
    origin: Point,
    string: &'a str,
    font: &'a MonoFont<'a>,
) -> Rectangle {
    let style = MonoTextStyleBuilder::new()
        .font(font)
        .text_color(TEXT_COLOR)
        .background_color(BACKGROUND_COLOR)
        .build();

    let text = Text::new(string, origin, style);
    let bounding_box = text.bounding_box();

    let text = {
        let mut text = text;
        let height: i32 = bounding_box.size.height.try_into().unwrap();
        text.position = Point::new(origin.x, origin.y + height);
        text
    };

    text.draw(display).ok();

    text.bounding_box()
}

pub fn draw_line(display: &mut impl Display, start: Point, end: Point) -> () {
    let style = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .stroke_color(TEXT_COLOR)
        .build();

    Line::new(start, end).into_styled(style).draw(display).ok();
}

pub fn draw_horizontal_line(display: &mut impl Display, x1: i32, x2: i32, y: i32) -> () {
    let start = Point::new(x1, y);
    let end = Point::new(x2, y);
    draw_line(display, start, end);
}

pub fn rgb565_to_rgb888(color: Rgb565) -> Rgb888 {
    let r = (color.r() as u16 * 527 + 23) >> 6;
    let g = (color.g() as u16 * 259 + 33) >> 6;
    let b = (color.b() as u16 * 527 + 23) >> 6;
    Rgb888::new(r as u8, g as u8, b as u8)
}

pub fn rgb888_to_rgb565(color: Rgb888) -> Rgb565 {
    let r = (color.r() as u16 * 249 + 1014) >> 11;
    let g = (color.g() as u16 * 253 + 505) >> 10;
    let b = (color.b() as u16 * 249 + 1014) >> 11;
    Rgb565::new(r as u8, g as u8, b as u8)
}
