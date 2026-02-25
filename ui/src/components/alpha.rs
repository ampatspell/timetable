use embedded_graphics::{
    Pixel,
    image::ImageDrawable,
    pixelcolor::{Rgb565, Rgb888},
    prelude::*,
    primitives::Rectangle,
};

use crate::components::{BACKGROUND_COLOR, utils::blend};

pub struct ImageAlpha<'a> {
    width: u32,
    height: u32,
    data: &'a [u8],
    process: &'a (dyn ProcessPixel<Rgb565> + 'static),
}

impl<'a> ImageAlpha<'a> {
    pub fn new(
        data: &'a [u8],
        width: u32,
        process: &'a (dyn ProcessPixel<Rgb565> + 'static),
    ) -> Self {
        let height = data.len() as u32 / width;
        Self {
            data,
            width,
            height,
            process,
        }
    }
}

impl<'a> OriginDimensions for ImageAlpha<'a> {
    fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }
}

impl<'a> ImageDrawable for ImageAlpha<'a> {
    type Color = Rgb565;

    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let area = Rectangle::new(Point::zero(), Size::new(self.width, self.height));
        self.draw_sub_image(target, &area)
    }

    fn draw_sub_image<D>(&self, target: &mut D, area: &Rectangle) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let width = self.width;
        let data = self.data;
        let process = self.process;
        target.draw_iter(AlphaPixelsIterator::new(data, width, area, process))
    }
}

pub struct AlphaPixelsIterator<'a> {
    data: &'a [u8],
    width: u32,
    point: Point,
    area: &'a Rectangle,
    process: &'a (dyn ProcessPixel<Rgb565> + 'static),
}

impl<'a> AlphaPixelsIterator<'a> {
    pub fn new(
        data: &'a [u8],
        width: u32,
        area: &'a Rectangle,
        process: &'a (dyn ProcessPixel<Rgb565> + 'static),
    ) -> Self {
        Self {
            data,
            width,
            point: area.top_left,
            area,
            process,
        }
    }
}

impl<'a> Iterator for AlphaPixelsIterator<'a> {
    type Item = Pixel<Rgb565>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.point;
        let width = self.width;
        let area = self.area;
        let top_left = area.top_left;
        let size = area.size;
        let data = self.data;
        let index = to_index(current, width);

        if current.y > top_left.y + size.height as i32 - 1 {
            return None;
        }

        if current.x == (size.width - 1) as i32 {
            self.point = Point::new(top_left.x, current.y + 1);
        } else {
            self.point = Point::new(current.x + 1, current.y);
        }

        let value = 255 - data[index as usize];
        let color = self.process.process_color(value);
        let point = Point::new(current.x - top_left.x, current.y - top_left.y);

        Some(Pixel { 0: point, 1: color })
    }
}

fn to_index(point: Point, width: u32) -> u32 {
    (point.y as u32 * width) + point.x as u32
}

pub trait ProcessPixel<C: PixelColor> {
    fn process_color(&self, alpha: u8) -> C;
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
