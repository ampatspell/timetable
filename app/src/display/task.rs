use crate::{
    channel::{NETWORK_CHANNEL, Network, VISUAL_CHANNEL, Visual},
    display::create::{CreateDisplayOptions, create_display},
    time::Time,
};
use defmt::info;
use embassy_time::{Duration, Timer};
use esp_hal::{Blocking, gpio::Output, spi::master::Spi};
use no_std_strings::{str12, str32};
use numtoa::NumToA;
use ui::{payload::BlockPayload, ui::UI};

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
        let message = VISUAL_CHANNEL.receive().await;
        match message {
            Visual::Time { time } => {
                ui.on_time(&mut display, time);
            }
            Visual::Weather { blocks } => {
                ui.on_weather(&mut display, blocks);
            }
            Visual::Timetable { block } => ui.on_timetable(&mut display, block),
            Visual::Message { message } => ui.on_message(&mut display, message),
        };
    }
}

#[embassy_executor::task]
pub async fn display_timer_task() {
    let mut time: Option<Time> = None;
    loop {
        let message = NETWORK_CHANNEL.receive().await;
        match message {
            Network::Time { time: next } => {
                time = Some(next);
            }
            Network::Tick => {
                time = match time {
                    Some(time) => {
                        let now = time.add_second();

                        let mut time = str12::new();
                        let mut buffer = [0u8; 20];
                        let mut push = |value: u8, string: &mut str12| {
                            let formatted = value.numtoa_str(10, &mut buffer);
                            if formatted.len() == 1 {
                                string.push("0");
                            }
                            string.push_str(formatted);
                        };

                        push(now.hours, &mut time);
                        time.push_str(":");
                        push(now.minutes, &mut time);
                        time.push_str(":");
                        push(now.seconds, &mut time);

                        VISUAL_CHANNEL.send(Visual::Time { time }).await;

                        Some(now)
                    }
                    _ => None,
                }
            }
            Network::Weather { weather } => {
                let blocks: [BlockPayload; 4] = [
                    BlockPayload {
                        icon: weather.icon,
                        lines: [weather.temperature, weather.description],
                    },
                    BlockPayload {
                        icon: str12::from("sun"),
                        lines: [weather.uv, str32::new()],
                    },
                    BlockPayload {
                        icon: str12::from("sunrise"),
                        lines: [weather.sunrise, str32::new()],
                    },
                    BlockPayload {
                        icon: str12::from("sunset"),
                        lines: [weather.sunset, str32::new()],
                    },
                ];
                VISUAL_CHANNEL.send(Visual::Weather { blocks }).await;
            }
            Network::Timetable { timetable } => {
                let block = BlockPayload {
                    icon: str12::from("bus-stop"),
                    lines: timetable,
                };
                VISUAL_CHANNEL.send(Visual::Timetable { block }).await;
            }
            Network::Message { message } => {
                VISUAL_CHANNEL.send(Visual::Message { message }).await;
            }
        }
    }
}
