#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use app::{
    configure::{ConfigureResponse, configure},
    display::{ConfigureDisplayOptions, configure_display},
    network::task::{ConfigureNetworkOptions, configure_network},
};
use defmt::info;
use embassy_executor::Spawner;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    info!("{}", info);
    loop {}
}

extern crate alloc;

esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) {
    info!("Hello");

    let ConfigureResponse {
        backlight,
        cs,
        dc,
        rst,
        spi,
        wifi,
    } = configure();

    configure_display(ConfigureDisplayOptions {
        spawner: &spawner,
        spi,
        rst,
        dc,
        cs,
        backlight,
    });

    configure_network(ConfigureNetworkOptions {
        spawner: &spawner,
        wifi,
    })
    .await;
}
