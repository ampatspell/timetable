use crate::{
    channel::{CHANNEL, Messages},
    display::create::{CreateDisplayOptions, create_display},
};
use defmt::info;
use embassy_time::{Duration, Timer};
use esp_hal::{Blocking, gpio::Output, spi::master::Spi};
use no_std_strings::str128;
use ui::{
    Display,
    components::icons::Icons,
    draw::{draw_content, draw_first_frame},
    payload::{Payload, Temperature, Tram, Weather, Wind},
};

pub struct DisplayTaskOptions {
    pub spi: Spi<'static, Blocking>,
    pub rst: Output<'static>,
    pub dc: Output<'static>,
    pub cs: Output<'static>,
    pub backlight: Output<'static>,
}

fn fake_draw(display: &mut impl Display, icons: &Icons) -> () {
    let payload = Payload {
        weather: Weather {
            temperature: Temperature {
                value: -5.7,
                description: str128::from("Snow grains fall"),
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

    draw_content(display, payload, icons);
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
    let icons = Icons::new();

    draw_first_frame(&mut display);

    let _backlight = {
        let mut backlight = backlight;
        backlight.set_high();
    };
    Timer::after(Duration::from_secs(1)).await;

    fake_draw(&mut display, &icons);

    loop {
        let message = CHANNEL.receive().await;
        let _ = match message {
            Messages::Update { payload } => payload,
        };
        // draw_content(&mut display, payload, &icons);
    }
}
