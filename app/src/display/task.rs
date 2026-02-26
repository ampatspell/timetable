use crate::{
    channel::{NETWORK_CHANNEL, NetworkMessages, UI_CHANNEL},
    display::create::{CreateDisplayOptions, create_display},
};
use chrono::{DateTime, TimeDelta, Timelike};
use chrono_tz::Tz;
use defmt::info;
use embassy_time::{Duration, Timer};
use esp_hal::{Blocking, gpio::Output, spi::master::Spi};
use no_std_strings::str8;
use numtoa::NumToA;
use ui::ui::UI;

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

    // loop {
    //     let message = NETWORK.receive().await;
    //     match message {
    //         NetworkMessages::Ping { .. } => {
    //             ui.update();
    //             ui.draw(&mut display);
    //         }
    //         NetworkMessages::Time { .. } => {}
    //     };
    // }
}

#[embassy_executor::task]
pub async fn display_timer_task() {
    let mut date_time: Option<DateTime<Tz>> = None;

    loop {
        let message = NETWORK_CHANNEL.receive().await;
        match message {
            NetworkMessages::Time { date_time: next } => {
                date_time = Some(next.with_timezone(&chrono_tz::Europe::Riga));
            }
            NetworkMessages::Tick => {
                date_time = match date_time {
                    Some(date_time) => {
                        let now = date_time.checked_add_signed(TimeDelta::seconds(1)).unwrap();
                        let values = now.time();

                        let mut time = str8::new();
                        let mut buffer = [0u8; 20];
                        let mut push = |value: u32, string: &mut str8| {
                            let formatted = value.numtoa_str(10, &mut buffer);
                            string.push_str(formatted);
                        };

                        push(values.hour(), &mut time);
                        time.push_str(":");
                        push(values.minute(), &mut time);
                        time.push_str(":");
                        push(values.second(), &mut time);

                        let s = time.to_str();
                        info!("{}", s);

                        UI_CHANNEL
                            .send(crate::channel::UiMessages::Time { time })
                            .await;

                        Some(now)
                    }
                    _ => None,
                }
            }
        }
    }
}
