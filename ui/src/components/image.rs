use embedded_graphics::{image::ImageRawLE, pixelcolor::Rgb565, prelude::*};

use crate::{Display, components::transparent::ImageTransparent};

pub fn draw_image(display: &mut impl Display) -> () {
    let raw = ImageRawLE::<Rgb565>::new(
        include_bytes!("../../../images/assets/tabler-icon-sun.raw"),
        24,
    );

    let image = ImageTransparent::new(raw, Rgb565::BLACK);
    image.draw(display).ok();
}
