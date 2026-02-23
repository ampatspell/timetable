use crate::{
    channel::{CHANNEL, Messages},
    display::create::{CreateDisplayOptions, create_display},
};
use defmt::info;
use embassy_time::{Duration, Timer};
use esp_hal::{Blocking, gpio::Output, spi::master::Spi};
use ui::draw::UI;

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
    let mut ui = UI::new();

    ui.prepare(&mut display);

    {
        let mut backlight = backlight;
        backlight.set_high();
    };

    Timer::after(Duration::from_secs(1)).await;

    loop {
        let message = CHANNEL.receive().await;
        match message {
            Messages::Ping { .. } => {
                ui.update();
                ui.draw(&mut display);
            }
            Messages::Time { .. } => {}
        };
    }
}

#[embassy_executor::task]
pub async fn display_timer_task() {
    loop {
        CHANNEL.send(Messages::Ping {}).await;
        Timer::after(Duration::from_secs(1)).await;
    }
}
