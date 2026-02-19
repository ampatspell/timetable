use embassy_executor::Spawner;
use esp_hal::{Blocking, gpio::Output, spi::master::Spi};

use crate::display::task::{DisplayTaskOptions, display_task};

mod create;
mod task;

pub struct ConfigureDisplayOptions<'a> {
    pub spawner: &'a Spawner,
    pub spi: Spi<'static, Blocking>,
    pub rst: Output<'static>,
    pub dc: Output<'static>,
    pub cs: Output<'static>,
    pub backlight: Output<'static>,
}

pub fn configure_display<'a>(options: ConfigureDisplayOptions<'a>) {
    let ConfigureDisplayOptions {
        cs,
        dc,
        rst,
        spi,
        spawner,
        backlight,
    } = options;

    spawner
        .spawn(display_task(DisplayTaskOptions {
            cs,
            dc,
            rst,
            spi,
            backlight,
        }))
        .ok();
}
