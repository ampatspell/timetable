use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embedded_graphics::{
    Drawable,
    mono_font::{MonoTextStyle, ascii::FONT_9X18_BOLD},
    pixelcolor::Rgb565,
    prelude::{DrawTarget, Point, RgbColor},
    text::Text,
};
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::{Blocking, delay::Delay, gpio::Output, spi::master::Spi};
use mipidsi::{
    Builder, Display,
    interface::SpiInterface,
    models::ST7789,
    options::{Orientation, Rotation},
};

use crate::channel::{CHANNEL, Messages};

// #define LCD_WIDTH 240
// #define LCD_HEIGHT 280

static BUFFER: static_cell::StaticCell<[u8; 512]> = static_cell::StaticCell::new();

struct CreateDisplayOptions {
    spi: Spi<'static, Blocking>,
    rst: Output<'static>,
    dc: Output<'static>,
    cs: Output<'static>,
}

fn create_display(
    opts: CreateDisplayOptions,
) -> Display<
    SpiInterface<
        'static,
        ExclusiveDevice<Spi<'static, Blocking>, Output<'static>, embedded_hal_bus::spi::NoDelay>,
        Output<'static>,
    >,
    ST7789,
    Output<'static>,
> {
    let CreateDisplayOptions { cs, dc, rst, spi } = opts;
    let buffer = BUFFER.uninit().write([0u8; 512]);
    let mut delay = Delay::new();
    let spi_device = ExclusiveDevice::new_no_delay(spi, cs).unwrap();
    let di = SpiInterface::new(spi_device, dc, buffer);
    let display = Builder::new(ST7789, di)
        .reset_pin(rst)
        .init(&mut delay)
        .unwrap();

    let display = {
        let mut display = display;
        display
            .set_orientation(Orientation::new().rotate(Rotation::Deg90))
            .unwrap();
        display
    };

    display
}

struct DisplayTaskOptions {
    spi: Spi<'static, Blocking>,
    rst: Output<'static>,
    dc: Output<'static>,
    cs: Output<'static>,
}

#[embassy_executor::task]
async fn display_task(opts: DisplayTaskOptions) {
    let DisplayTaskOptions { cs, dc, rst, spi } = opts;
    info!("Start display_task");

    let mut display = create_display(CreateDisplayOptions { spi, rst, dc, cs });
    display.clear(Rgb565::BLACK).ok();

    let position = Point::new(40, 30);
    let style = MonoTextStyle::new(&FONT_9X18_BOLD, Rgb565::WHITE);

    Timer::after(Duration::from_secs(5)).await;

    Text::new("Connecting…", position, style)
        .draw(&mut display)
        .ok();

    loop {
        let message = CHANNEL.receive().await;

        let str = match message {
            Messages::Update { text } => Some(text),
        };

        let value = str.unwrap();

        display.clear(Rgb565::BLACK).ok();
        Text::new(value.to_str(), position, style)
            .draw(&mut display)
            .ok();
    }
}

pub struct ConfigureDisplayOptions<'a> {
    pub spawner: &'a Spawner,
    pub spi: Spi<'static, Blocking>,
    pub rst: Output<'static>,
    pub dc: Output<'static>,
    pub cs: Output<'static>,
}

pub fn configure_display<'a>(options: ConfigureDisplayOptions<'a>) {
    let ConfigureDisplayOptions {
        cs,
        dc,
        rst,
        spi,
        spawner,
    } = options;

    spawner
        .spawn(display_task(DisplayTaskOptions { cs, dc, rst, spi }))
        .ok();
}
