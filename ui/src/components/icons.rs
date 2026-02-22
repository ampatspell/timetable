use defmt::info;
use embedded_graphics::{
    image::{Image, ImageRaw, ImageRawLE},
    pixelcolor::{Rgb565, Rgb888, raw::LittleEndian},
    prelude::*,
};
use static_cell::StaticCell;

use crate::{
    Display,
    components::{
        BACKGROUND_COLOR,
        transparent::{ImageTransparent, ProcessColor},
        utils::{rgb565_to_rgb888, rgb888_to_rgb565},
    },
};

pub struct Icons<'a> {
    map: [Icon<'a>; 6],
}
static PROCESS: StaticCell<BlendBackground> = StaticCell::new();

impl<'a> Icons<'a> {
    pub fn new() -> Self {
        let process = PROCESS.uninit().write(BlendBackground::new());
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
    image: ImageTransparent<'a, ImageRaw<'a, Rgb565, LittleEndian>>,
}

impl<'a> Icon<'a> {
    pub fn new(
        name: &'a str,
        data: &'a [u8],
        width: u32,
        process: &'a (dyn ProcessColor<Rgb565> + 'static),
    ) -> Self {
        let raw = ImageRawLE::<Rgb565>::new(data, width);
        let image = ImageTransparent::new(raw, Rgb565::BLACK, process);
        Self { name, image }
    }

    pub fn draw_at(&self, display: &mut impl Display, position: Point) -> () {
        let image = Image::new(&self.image, position);
        let result = image.draw(display);
        result.ok();
    }
}

pub struct BlendBackground {
    background: Rgb888,
}

impl BlendBackground {
    pub fn new() -> Self {
        Self {
            background: rgb565_to_rgb888(BACKGROUND_COLOR),
        }
    }

    pub fn blend(&self, value: f32) -> Rgb888 {
        let background = self.background;
        let calc = |channel: u8| -> u8 { (channel as f32 * value) as u8 };
        let r = calc(background.r());
        let g = calc(background.g());
        let b = calc(background.b());
        Rgb888::new(r, g, b)
    }

    pub fn invert(&self, color: Rgb888) -> Rgb888 {
        let calc = |channel: u8| -> u8 { 255 - channel };
        let r = calc(color.r());
        let g = calc(color.g());
        let b = calc(color.b());
        Rgb888::new(r, g, b)
    }
}

impl ProcessColor<Rgb565> for BlendBackground {
    fn process_color(&self, color: Rgb565) -> Rgb565 {
        // let background = self.background;

        let c = self.invert(rgb565_to_rgb888(color));
        let value = ((c.r() as f32 + c.g() as f32 + c.b() as f32) / 3.0) / 255.0;
        rgb888_to_rgb565(self.blend(value))

        // Rgb565::new(r as u8, g as u8, b as u8)
    }
}
