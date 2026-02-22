use crate::{Display, components::utils::draw_text, payload::Tram};
use core::ops::Add;
use embedded_graphics::prelude::Point;
use profont::PROFONT_18_POINT;

pub struct TramOptions<'a> {
    pub tram: &'a Tram,
    pub origin: Point,
}

pub fn draw_tram(display: &mut impl Display, opts: TramOptions) -> i32 {
    let TramOptions { tram, origin } = opts;

    let string = &tram.time;
    let font = PROFONT_18_POINT;

    let rec = draw_text(display, origin, string, &font);

    rec.size.height.try_into().unwrap()
}

pub struct ScheduleOptions<'a> {
    pub trams: &'a [Tram; 2],
    pub origin: Point,
}

pub fn draw_schedule(display: &mut impl Display, opts: ScheduleOptions) -> () {
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
