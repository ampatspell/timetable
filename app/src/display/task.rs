use defmt::info;
use embassy_time::{Duration, Timer};
use embedded_graphics::{
    Drawable,
    mono_font::{MonoTextStyle, ascii::FONT_9X18_BOLD},
    pixelcolor::Rgb565,
    prelude::{Dimensions, DrawTarget, Point, RgbColor},
    text::Text,
};
use esp_hal::{Blocking, gpio::Output, spi::master::Spi};
use no_std_strings::str128;
use ui::{
    draw::{draw_content, draw_first_frame},
    payload::{Payload, Temperature, Tram, Weather, Wind},
};

use crate::{
    channel::{CHANNEL, Messages},
    display::create::{CreateDisplayOptions, create_display},
};

pub struct DisplayTaskOptions {
    pub spi: Spi<'static, Blocking>,
    pub rst: Output<'static>,
    pub dc: Output<'static>,
    pub cs: Output<'static>,
    pub backlight: Output<'static>,
}

fn fake_draw<D>(display: &mut D) -> ()
where
    D: DrawTarget<Color = Rgb565> + Dimensions,
{
    let payload = Payload {
        weather: Weather {
            temperature: Temperature {
                value: -5.7,
                description: str128::from("Snow grains falling."),
            },
            wind: Wind {
                speed: 12.1,
                direction: 37,
            },
        },
        trams: (
            Tram {
                time: str128::from("03:55"),
                adjustment: -103,
            },
            Tram {
                time: str128::from("04:21"),
                adjustment: 0,
            },
        )
            .into(),
    };

    draw_content(display, payload);
}

#[embassy_executor::task]
pub async fn display_task(opts: DisplayTaskOptions) {
    let DisplayTaskOptions {
        cs,
        dc,
        rst,
        spi,
        backlight,
    } = opts;
    info!("Start display_task");

    let mut display = create_display(CreateDisplayOptions { spi, rst, dc, cs });

    draw_first_frame(&mut display);

    let _backlight = {
        let mut backlight = backlight;
        backlight.set_high();
    };
    Timer::after(Duration::from_secs(1)).await;

    // let position = Point::new(40, 30);
    // let style = MonoTextStyle::new(&FONT_9X18_BOLD, Rgb565::WHITE);

    // Text::new("Connecting...", position, style)
    //     .draw(&mut display)
    //     .ok();

    fake_draw(&mut display);

    loop {
        let message = CHANNEL.receive().await;
        let payload = match message {
            Messages::Update { payload } => payload,
        };
        draw_content(&mut display, payload);
    }
}
