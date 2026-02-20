use crate::{
    components::{
        BACKGROUND_COLOR,
        schedule::{ScheduleOptions, draw_schedule},
        temperature::{WeatherOptions, draw_weather},
    },
    payload::Payload,
};
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::{Dimensions, DrawTarget, Point},
};

// const WIDTH: i32 = 280;
// const HEIGHT: i32 = 240;

pub fn draw_content<D>(display: &mut D, payload: Payload) -> ()
where
    D: DrawTarget<Color = Rgb565> + Dimensions,
{
    draw_weather(
        display,
        WeatherOptions {
            temperature: &payload.weather.temperature,
            origin: Point::new(40, 40),
        },
    );

    draw_schedule(
        display,
        ScheduleOptions {
            trams: &payload.trams,
            origin: Point::new(40, 80),
        },
    );
}

pub fn draw_first_frame<D>(display: &mut D) -> ()
where
    D: DrawTarget<Color = Rgb565> + Dimensions,
{
    display.clear(BACKGROUND_COLOR).ok();
}
