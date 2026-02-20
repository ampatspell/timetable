use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::{Dimensions, DrawTarget, Point},
};

use crate::payload::Tram;

pub struct ScheduleOptions<'a> {
    pub trams: &'a [Tram; 2],
    pub origin: Point,
}

pub fn draw_schedule<D>(display: &mut D, opts: ScheduleOptions) -> ()
where
    D: DrawTarget<Color = Rgb565> + Dimensions,
{
}
