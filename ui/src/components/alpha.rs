use embedded_graphics::{
    Pixel, image::ImageDrawable, pixelcolor::Rgb565, prelude::*, primitives::Rectangle,
};

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
        let width = self.width;
        let data = self.data;
        let process = self.process;
        target.draw_iter(AlphaPixelsIterator::new(width, data, process))
    }

    fn draw_sub_image<D>(&self, target: &mut D, _area: &Rectangle) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        self.draw(target)
    }
}

pub struct AlphaPixelsIterator<'a> {
    width: u32,
    data: &'a [u8],
    point: Point,
    process: &'a (dyn ProcessPixel<Rgb565> + 'static),
}

impl<'a> AlphaPixelsIterator<'a> {
    pub fn new(
        width: u32,
        data: &'a [u8],
        process: &'a (dyn ProcessPixel<Rgb565> + 'static),
    ) -> Self {
        Self {
            width,
            data,
            point: Point::new(0, 0),
            process,
        }
    }
}

impl<'a> Iterator for AlphaPixelsIterator<'a> {
    type Item = Pixel<Rgb565>;

    fn next(&mut self) -> Option<Self::Item> {
        let point = self.point;
        let width = self.width;
        let data = self.data;
        let index = to_index(point, width);

        if index > data.len() as u32 - 1 {
            return None;
        }

        let value = 255 - data[index as usize];
        let color = self.process.process_color(value);

        if point.x == width as i32 {
            self.point = Point::new(0, point.y + 1);
        } else {
            self.point = Point::new(point.x + 1, point.y);
        }

        Some(Pixel { 0: point, 1: color })
    }
}

fn to_index(point: Point, width: u32) -> u32 {
    (point.y as u32 * width) + point.x as u32
}

pub trait ProcessPixel<C: PixelColor> {
    fn process_color(&self, alpha: u8) -> C;
}
