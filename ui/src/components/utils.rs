use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyleBuilder},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};
use no_std_strings::str32;

use crate::components::{BACKGROUND_COLOR, TEXT_COLOR};

pub struct TextOptions<'a> {
    pub origin: Point,
    pub string: &'a str,
    pub font: &'a MonoFont<'a>,
}

pub fn draw_text<D>(display: &mut D, opts: TextOptions) -> ()
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
}

pub fn float_to_string(value: f32) -> str32 {
    let mut buffer = ryu::Buffer::new();
    let value = buffer.format(value);
    str32::from(value)
}
