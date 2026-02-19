use embedded_graphics::{
    Drawable,
    mono_font::{MonoTextStyle, ascii::FONT_9X18_BOLD},
    pixelcolor::Rgb565,
    prelude::{Dimensions, DrawTarget, Point, RgbColor},
    primitives::{PrimitiveStyle, StyledDrawable},
    text::Text,
};
use no_std_strings::str32;

use crate::payload::Payload;

const WIDTH: i32 = 280;
const HEIGHT: i32 = 240;

pub fn draw_content<D>(display: &mut D, payload: Payload) -> ()
where
    D: DrawTarget<Color = Rgb565> + Dimensions,
{
    let mut buffer = ryu::Buffer::new();
    let temperature = buffer.format(payload.weather.temperature.value);
    let mut st = str32::from(temperature);
    st.push_str("C");

    let position = Point::new(40, 30);
    let style = MonoTextStyle::new(&FONT_9X18_BOLD, Rgb565::new(5, 11, 5));
    let stroke = PrimitiveStyle::with_stroke(Rgb565::RED, 1);

    display.clear(Rgb565::new(21, 62, 0)).ok();

    let text = Text::new(&st, position, style);
    let bb = text.bounding_box();
    // let x: i32 = (WIDTH / 2) + (bb.width / 2);
    // let y: i32 = (HEIGHT / 2) + (bb.height / 2);
    // text.position = Point::new(x, y);
    text.draw(display).ok();
    bb.draw_styled(&stroke, display).ok();
}
