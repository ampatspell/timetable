use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyleBuilder},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::Rectangle,
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
    let style = MonoTextStyleBuilder::new()
        .font(opts.font)
        .text_color(TEXT_COLOR)
        .background_color(BACKGROUND_COLOR)
        .build();
    let text = Text::new(opts.string, opts.origin, style);
    text.draw(display).ok();
    let bounding_box = text.bounding_box();

    bounding_box
}

pub fn float_to_string(value: f32) -> str32 {
    let mut buffer = ryu::Buffer::new();
    let value = buffer.format(value);
    str32::from(value)
}

pub fn sign32(value: u32) -> i32 {
    value.try_into().unwrap()
}
