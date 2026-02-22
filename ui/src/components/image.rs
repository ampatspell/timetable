use embedded_graphics::{
    image::{Image, ImageRawLE},
    pixelcolor::Rgb565,
    prelude::*,
};

use crate::{Display, components::transparent::ImageTransparent};

pub fn draw_image(display: &mut impl Display) -> () {
    let raw = ImageRawLE::<Rgb565>::new(
        include_bytes!("../../../images/assets/tabler-icon-sun.raw"),
        24,
    );

    let transparent = ImageTransparent::new(raw, Rgb565::BLACK);
    let image = Image::new(&transparent, Point::new(10, 10));

    image.draw(display).ok();
}
