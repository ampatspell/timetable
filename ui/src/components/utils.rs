use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyleBuilder},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Line, PrimitiveStyleBuilder, Rectangle},
    text::Text,
};
use no_std_strings::str32;

use crate::components::{BACKGROUND_COLOR, TEXT_COLOR};

pub struct TextOptions<'a> {
    pub origin: Point,
    pub string: &'a str,
    pub font: &'a MonoFont<'a>,
}

pub fn draw_text<D>(display: &mut D, opts: TextOptions) -> Rectangle
where
    D: DrawTarget<Color = Rgb565> + Dimensions,
{
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

pub fn float_to_string(value: f32) -> str32 {
    let mut buffer = ryu::Buffer::new();
    let value = buffer.format(value);
    str32::from(value)
}

pub fn draw_line<D>(display: &mut D, start: Point, end: Point) -> ()
where
    D: DrawTarget<Color = Rgb565> + Dimensions,
{
    let style = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .stroke_color(TEXT_COLOR)
        .build();

    Line::new(start, end).into_styled(style).draw(display).ok();
}

pub fn draw_horizontal_line<D>(display: &mut D, x1: i32, x2: i32, y: i32) -> ()
where
    D: DrawTarget<Color = Rgb565> + Dimensions,
{
    let start = Point::new(x1, y);
    let end = Point::new(x2, y);
    draw_line(display, start, end);
}
