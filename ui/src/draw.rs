use crate::{
    Display,
    components::{BACKGROUND_COLOR, block::Blocks, icons::Icons},
    payload::Payload,
};
use embedded_graphics::prelude::*;

// const WIDTH: i32 = 280;
// const HEIGHT: i32 = 240;

pub fn draw_content<'a>(display: &mut impl Display, payload: Payload, icons: &'a Icons<'a>) -> () {
    let blocks = Blocks::new(Point::new(35, 25), icons);
    blocks.draw(display);
}

pub fn draw_first_frame(display: &mut impl Display) -> () {
    display.clear(BACKGROUND_COLOR).ok();
}
