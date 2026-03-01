use defmt::info;
use embedded_graphics::{image::Image, pixelcolor::Rgb565, prelude::*, primitives::Rectangle};
use static_cell::StaticCell;

use crate::{
    Display,
    components::{
        BACKGROUND_COLOR,
        alpha::{BlendInBackground, ImageAlpha, ProcessPixel},
    },
};

pub struct Icons<'a> {
    map: [Icon<'a>; 7],
}

impl<'a> Icons<'a> {
    pub fn new() -> Self {
        let process = {
            static CELL: StaticCell<BlendInBackground> = StaticCell::new();
            CELL.init(BlendInBackground::new(BACKGROUND_COLOR))
        };
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
            Icon::new(
                "cat",
                include_bytes!("../../../images/assets/lucide-cat.raw"),
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

    pub fn draw_sub_image_at(
        &self,
        display: &mut impl Display,
        name: &str,
        position: Point,
        area: Rectangle,
    ) {
        let icon = self.map.iter().find(|icon| icon.name.eq(name));
        match icon {
            Some(icon) => icon.draw_sub_image_at(display, position, area),
            None => info!("Icon {} was not found", name),
        }
    }
}

pub struct Icon<'a> {
    pub name: &'a str,
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
        image.draw(display).ok();
    }

    pub fn draw_sub_image_at(
        &self,
        display: &mut impl Display,
        position: Point,
        area: Rectangle,
    ) -> () {
        let drawable = self.image.sub_image(&area);
        let image = Image::new(&drawable, position);
        image.draw(display).ok();
    }
}
