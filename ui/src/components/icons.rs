use defmt::info;
use embedded_graphics::{
    image::{Image, ImageRaw, ImageRawLE},
    pixelcolor::{Rgb565, Rgb888, raw::LittleEndian},
    prelude::*,
};
use lab::Lab;
use static_cell::StaticCell;

use crate::{
    Display,
    components::{
        BACKGROUND_COLOR,
        transparent::{ProcessPixelColor, ProcessedImage},
        utils::invert,
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
    image: ProcessedImage<'a, ImageRaw<'a, Rgb565, LittleEndian>>,
}

impl<'a> Icon<'a> {
    pub fn new(
        name: &'a str,
        data: &'a [u8],
        width: u32,
        process: &'a (dyn ProcessPixelColor<Rgb565> + 'static),
    ) -> Self {
        let raw = ImageRawLE::<Rgb565>::new(data, width);
        let image = ProcessedImage::new(raw, process);
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

    pub fn blend(&self, color: Rgb888) -> Rgb888 {
        let background = self.background;

        if color.r() == 255 {
            return background;
        }

        let c_background = [background.r(), background.g(), background.b()];
        let l_background = Lab::from_rgb(&c_background);

        let c_color = [color.r(), color.g(), color.b()];
        let l_color = Lab::from_rgb(&c_color);

        let l = Lab {
            l: (l_background.l + l_color.l) / 2.0,
            a: (l_background.a + l_color.a) / 2.0,
            b: (l_background.b + l_color.b) / 2.0,
        };

        let [r, g, b] = l.to_rgb();
        Rgb888::new(r, g, b)
    }
}

impl ProcessPixelColor<Rgb565> for BlendInBackground {
    fn process_color(&self, _color: Rgb565) -> Rgb565 {
        let color = invert(Rgb888::from(_color));
        Rgb565::from(self.blend(color))
    }
}
