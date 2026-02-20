use core::ops::Add;

use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Circle, PrimitiveStyle},
};
use profont::PROFONT_18_POINT;

use crate::{
    components::{
        TEXT_COLOR,
        utils::{TextOptions, draw_text, float_to_string, sign32},
    },
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

    {
        // -19.2 C
        let string = float_to_string(temperature.value);
        let string = {
            let mut string = string;
            string.push(" C");
            string
        };

        let font = PROFONT_18_POINT;
        let bounding_box = {
            let bounding_box = draw_text(
                display,
                TextOptions {
                    origin: origin.add(Point::new(0, 0)),
                    string: &string,
                    font: &font,
                },
            );

            bounding_box
        };

        // °
        {
            let width: i32 = sign32(bounding_box.size.width);
            let height: i32 = sign32(bounding_box.size.height);
            let origin = Point::new(bounding_box.top_left.x + width + 2, origin.y - height + 6);
            Circle::new(origin, 7)
                .into_styled(PrimitiveStyle::with_stroke(TEXT_COLOR, 2))
                .draw(display)
                .ok();
        }
    }

    {
        draw_text(
            display,
            TextOptions {
                origin: origin.add(Point::new(0, 15)),
                string: &temperature.description,
                font: &profont::PROFONT_14_POINT,
            },
        );
    }
}
