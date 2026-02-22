use defmt::info;
use embedded_graphics::{
    image::Image,
    pixelcolor::{Rgb565, Rgb888},
    prelude::*,
};
use static_cell::StaticCell;

use crate::{
    Display,
    components::{
        BACKGROUND_COLOR,
        alpha::{ImageAlpha, ProcessPixel},
        utils::blend,
    },
};

pub struct Icons<'a> {
    map: [Icon<'a>; 6],
}
static PROCESS: StaticCell<BlendInBackground> = StaticCell::new();

impl<'a> Icons<'a> {
    pub fn new() -> Self {
        let process = PROCESS.uninit().write(BlendInBackground::new());
        let map = [
            Icon::new(
                "bus-stop",
                include_bytes!("../../../images/assets/tabler-icon-bus-stop.raw"),
                24,
                process,
            ),
            Icon::new(
                "clock",
                include_bytes!("../../../images/assets/tabler-icon-clock.raw"),
                24,
                process,
            ),
            Icon::new(
                "cloud-snow",
                include_bytes!("../../../images/assets/tabler-icon-cloud-snow.raw"),
                24,
                process,
            ),
            Icon::new(
                "sun",
                include_bytes!("../../../images/assets/tabler-icon-sun.raw"),
                24,
                process,
            ),
            Icon::new(
                "sunrise",
                include_bytes!("../../../images/assets/tabler-icon-sunrise.raw"),
                24,
                process,
            ),
            Icon::new(
                "sunset",
                include_bytes!("../../../images/assets/tabler-icon-sunset.raw"),
                24,
                process,
            ),
        ];
        Self { map }
    }

    pub fn draw_at(&self, display: &mut impl Display, name: &str, position: Point) -> () {
        let icon = self.map.iter().find(|icon| icon.name.eq(name));
        match icon {
            Some(icon) => icon.draw_at(display, position),
            None => info!("Icon {} was not found", name),
        }
    }
}

pub struct Icon<'a> {
    pub name: &'a str,
    // image: ProcessedImage<'a, ImageRaw<'a, Rgb565, LittleEndian>>,
    image: ImageAlpha<'a>,
}

impl<'a> Icon<'a> {
    pub fn new(
        name: &'a str,
        data: &'a [u8],
        width: u32,
        process: &'a (dyn ProcessPixel<Rgb565> + 'static),
    ) -> Self {
        let image = ImageAlpha::new(data, width, process);
        Self { name, image }
    }

    pub fn draw_at(&self, display: &mut impl Display, position: Point) -> () {
        let image = Image::new(&self.image, position);
        let result = image.draw(display);
        result.ok();
    }
}

pub struct BlendInBackground {
    background: Rgb888,
}

impl BlendInBackground {
    pub fn new() -> Self {
        Self {
            background: Rgb888::from(BACKGROUND_COLOR),
        }
    }
}

impl ProcessPixel<Rgb565> for BlendInBackground {
    fn process_color(&self, alpha: u8) -> Rgb565 {
        Rgb565::from(blend(self.background, Rgb888::WHITE, 255 - alpha))
    }
}
