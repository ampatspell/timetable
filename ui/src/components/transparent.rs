use embedded_graphics::{
    Pixel,
    image::ImageDrawable,
    prelude::{Dimensions, DrawTarget, OriginDimensions, PixelColor, Size},
    primitives::Rectangle,
};

pub struct ImageTransparent<'a, T: ImageDrawable> {
    source: T,
    transparent_color: T::Color,
    process: &'a (dyn ProcessColor<T::Color> + 'a),
}

impl<'a, T: ImageDrawable> ImageTransparent<'a, T> {
    pub fn new(
        source: T,
        transparent_color: T::Color,
        process: &'a (dyn ProcessColor<T::Color> + 'a),
    ) -> Self {
        ImageTransparent {
            source,
            transparent_color,
            process,
        }
    }
}

impl<'a, T: ImageDrawable> OriginDimensions for ImageTransparent<'a, T> {
    fn size(&self) -> Size {
        self.source.size()
    }
}

impl<'a, T: ImageDrawable> ImageDrawable for ImageTransparent<'a, T> {
    type Color = T::Color;

    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let mut draw_target = TransparentDrawTarget {
            target,
            transparent_color: self.transparent_color,
            process: self.process,
        };
        self.source.draw(&mut draw_target)
    }

    fn draw_sub_image<D>(&self, target: &mut D, area: &Rectangle) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let mut draw_target = TransparentDrawTarget {
            target,
            transparent_color: self.transparent_color,
            process: self.process,
        };
        self.source.draw_sub_image(&mut draw_target, area)
    }
}

struct TransparentDrawTarget<'a, T: DrawTarget> {
    target: &'a mut T,
    transparent_color: T::Color,
    process: &'a (dyn ProcessColor<T::Color> + 'a),
}

impl<'a, T: DrawTarget> Dimensions for TransparentDrawTarget<'a, T> {
    fn bounding_box(&self) -> Rectangle {
        self.target.bounding_box()
    }
}

impl<'a, T: DrawTarget> DrawTarget for TransparentDrawTarget<'a, T> {
    type Color = T::Color;
    type Error = T::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let process = self.process;
        self.target.draw_iter(
            pixels
                .into_iter()
                .filter(|pixel| pixel.1 != self.transparent_color)
                .map(|pixel| {
                    let color = process.process_color(pixel.1);
                    Pixel {
                        0: pixel.0,
                        1: color,
                    }
                }),
        )
    }
}

pub trait ProcessColor<C: PixelColor> {
    fn process_color(&self, color: C) -> C;
}
