use crate::{
    Display,
    components::{
        BACKGROUND_COLOR,
        image::draw_image,
        schedule::{ScheduleOptions, draw_schedule},
        temperature::{WeatherOptions, draw_weather},
        utils::draw_horizontal_line,
    },
    payload::Payload,
};
use embedded_graphics::prelude::Point;

const WIDTH: i32 = 280;
const HEIGHT: i32 = 240;

pub fn draw_content(display: &mut impl Display, payload: Payload) -> () {
    let padding = 20;

    draw_image(display);

    // draw_weather(
    //     display,
    //     WeatherOptions {
    //         temperature: &payload.weather.temperature,
    //         origin: Point::new(padding, padding),
    //     },
    // );

    // // 240px width is not real, height also
    // draw_horizontal_line(display, 0, WIDTH + 50, HEIGHT / 2);

    // draw_schedule(
    //     display,
    //     ScheduleOptions {
    //         trams: &payload.trams,
    //         origin: Point::new(padding, HEIGHT / 2 + padding),
    //     },
    // );
}

pub fn draw_first_frame(display: &mut impl Display) -> () {
    display.clear(BACKGROUND_COLOR).ok();
}
