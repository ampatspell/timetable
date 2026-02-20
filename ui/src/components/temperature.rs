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

pub fn draw_temperature_value<D>(display: &mut D, opts: WeatherOptions) -> ()
where
    D: DrawTarget<Color = Rgb565> + Dimensions,
{
    let string = float_to_string(opts.temperature.value);
    {
        let mut string = string;
        string.push_str(" C");
    }

    draw_text(
        display,
        TextOptions {
            origin: opts.origin,
            string: &string,
            font: &PROFONT_18_POINT,
        },
    );
}

pub fn draw_temperature_description<D>(display: &mut D, opts: WeatherOptions) -> ()
where
    D: DrawTarget<Color = Rgb565> + Dimensions,
{
    draw_text(
        display,
        TextOptions {
            origin: opts.origin,
            string: &opts.temperature.description,
            font: &profont::PROFONT_14_POINT,
        },
    );
}

pub fn draw_weather<D>(display: &mut D, opts: WeatherOptions) -> ()
where
    D: DrawTarget<Color = Rgb565> + Dimensions,
{
    let WeatherOptions {
        temperature,
        origin,
    } = opts;

    draw_temperature_value(
        display,
        WeatherOptions {
            temperature: &temperature,
            origin: origin.add(Point::new(0, 0)),
        },
    );
    draw_temperature_description(
        display,
        WeatherOptions {
            temperature: &temperature,
            origin: origin.add(Point::new(0, 15)),
        },
    );
}
