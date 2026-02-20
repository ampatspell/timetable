use core::ops::Add;

use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::{Dimensions, DrawTarget, Point},
};
use profont::PROFONT_24_POINT;

use crate::{components::utils::draw_text, payload::Tram};

pub struct TramOptions<'a> {
    pub tram: &'a Tram,
    pub origin: Point,
}

pub fn draw_tram<D>(display: &mut D, opts: TramOptions) -> i32
where
    D: DrawTarget<Color = Rgb565> + Dimensions,
{
    let TramOptions { tram, origin } = opts;

    let string = &tram.time;
    let font = PROFONT_24_POINT;

    let rec = draw_text(
        display,
        super::utils::TextOptions {
            origin,
            string,
            font: &font,
        },
    );

    rec.size.height.try_into().unwrap()
}

pub struct ScheduleOptions<'a> {
    pub trams: &'a [Tram; 2],
    pub origin: Point,
}

pub fn draw_schedule<D>(display: &mut D, opts: ScheduleOptions) -> ()
where
    D: DrawTarget<Color = Rgb565> + Dimensions,
{
    let ScheduleOptions { trams, origin } = opts;

    let mut y = -4;
    for tram in trams.iter() {
        y += draw_tram(
            display,
            TramOptions {
                tram: &tram,
                origin: origin.add(Point::new(0, y)),
            },
        );
    }
}
