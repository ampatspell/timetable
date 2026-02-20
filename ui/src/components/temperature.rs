use core::ops::Add;

use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use profont::PROFONT_18_POINT;

use crate::{
    components::utils::{TextOptions, draw_text, float_to_string},
    payload::Temperature,
};

pub struct WeatherOptions<'a> {
    pub temperature: &'a Temperature,
    pub origin: Point,
}

pub fn draw_weather<D>(display: &mut D, opts: WeatherOptions) -> ()
where
    D: DrawTarget<Color = Rgb565> + Dimensions,
{
    let WeatherOptions {
        temperature,
        origin,
    } = opts;

    let string = float_to_string(temperature.value);
    {
        let mut string = string;
        string.push_str(" C");
    }

    draw_text(
        display,
        TextOptions {
            origin: origin.add(Point::new(0, 0)),
            string: &string,
            font: &PROFONT_18_POINT,
        },
    );

    draw_text(
        display,
        TextOptions {
            origin: origin.add(Point::new(0, 15)),
            string: &temperature.description,
            font: &profont::PROFONT_14_POINT,
        },
    );
}
