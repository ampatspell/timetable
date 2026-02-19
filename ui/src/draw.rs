use embedded_graphics::{
    Drawable,
    mono_font::{MonoTextStyle, ascii::FONT_9X18_BOLD},
    pixelcolor::Rgb565,
    prelude::{Dimensions, DrawTarget, Point, RgbColor},
    text::Text,
};
use no_std_strings::str32;

use crate::payload::Payload;

pub fn draw_content<D>(display: &mut D, payload: Payload) -> ()
where
    D: DrawTarget<Color = Rgb565> + Dimensions,
{
    let mut buffer = ryu::Buffer::new();
    let temperature = buffer.format(payload.weather.temperature.value);
    let mut st = str32::from(temperature);
    st.push_str("C");

    let position = Point::new(40, 30);
    let style = MonoTextStyle::new(&FONT_9X18_BOLD, Rgb565::WHITE);

    display.clear(Rgb565::RED).ok();
    Text::new(&st, position, style).draw(display).ok();
}
