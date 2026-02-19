use defmt::info;
use embassy_time::{Duration, Timer};
use embedded_graphics::{
    Drawable,
    mono_font::{MonoTextStyle, ascii::FONT_9X18_BOLD},
    pixelcolor::Rgb565,
    prelude::{DrawTarget, Point, RgbColor},
    text::Text,
};
use esp_hal::{Blocking, gpio::Output, spi::master::Spi};
use no_std_strings::str32;

use crate::{
    channel::{CHANNEL, Messages},
    display::create::{CreateDisplayOptions, create_display},
};

// #define LCD_WIDTH 240
// #define LCD_HEIGHT 280

pub struct DisplayTaskOptions {
    pub spi: Spi<'static, Blocking>,
    pub rst: Output<'static>,
    pub dc: Output<'static>,
    pub cs: Output<'static>,
    pub backlight: Output<'static>,
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
    display.clear(Rgb565::BLACK).ok();

    let _backlight = {
        let mut backlight = backlight;
        backlight.set_high();
    };

    let position = Point::new(40, 30);
    let style = MonoTextStyle::new(&FONT_9X18_BOLD, Rgb565::WHITE);

    Timer::after(Duration::from_secs(5)).await;

    Text::new("Connecting…", position, style)
        .draw(&mut display)
        .ok();

    loop {
        let message = CHANNEL.receive().await;

        let payload = match message {
            Messages::Update { payload } => payload,
        };

        let mut buffer = ryu::Buffer::new();
        let temperature = buffer.format(payload.weather.temperature.value);
        let mut st = str32::from(temperature);
        st.push_str("°C");

        display.clear(Rgb565::BLACK).ok();
        Text::new(&st, position, style).draw(&mut display).ok();
    }
}
