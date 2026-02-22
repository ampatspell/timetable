use crate::{
    Display,
    components::{
        TEXT_COLOR,
        utils::{draw_text, float_to_string},
    },
    payload::Temperature,
};
use core::ops::Add;
use embedded_graphics::{
    prelude::*,
    primitives::{Circle, PrimitiveStyle},
};
use profont::PROFONT_24_POINT;

pub struct WeatherOptions<'a> {
    pub temperature: &'a Temperature,
    pub origin: Point,
}

pub fn draw_weather(display: &mut impl Display, opts: WeatherOptions) -> () {
    let WeatherOptions {
        temperature,
        origin: _origin,
    } = opts;

    let origin = _origin.add(Point::new(0, -4));

    {
        // -19.2 C
        let string = float_to_string(temperature.value);
        let string = {
            let mut string = string;
            string.push(" C");
            string
        };

        let font = PROFONT_24_POINT;
        let bounding_box = {
            let bounding_box = draw_text(display, origin, &string, &font);

            bounding_box
        };

        // °
        {
            let width: i32 = bounding_box.size.width.try_into().unwrap();
            let origin = Point::new(bounding_box.top_left.x + width + 2, origin.y + 5);

            Circle::new(origin, 7)
                .into_styled(PrimitiveStyle::with_stroke(TEXT_COLOR, 2))
                .draw(display)
                .ok();
        }
    }

    {
        draw_text(
            display,
            origin.add(Point::new(0, 26)),
            &temperature.description,
            &profont::PROFONT_14_POINT,
        );
    }
}
