use crate::{
    Display,
    components::{BACKGROUND_COLOR, TEXT_COLOR},
};
use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyleBuilder},
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

pub struct TextOptions<'a> {
    pub origin: Point,
    pub string: &'a str,
    pub font: &'a MonoFont<'a>,
}

pub fn draw_text(display: &mut impl Display, opts: TextOptions) -> Rectangle {
    let TextOptions {
        origin,
        string,
        font,
    } = opts;

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
