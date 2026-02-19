use esp_hal::Blocking;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::peripherals::WIFI;
use esp_hal::spi::Mode;
use esp_hal::spi::master::{Config, Spi};
use esp_hal::time::Rate;
use esp_hal::timer::timg::TimerGroup;

pub struct ConfigureResponse {
    pub wifi: WIFI<'static>,
    pub spi: Spi<'static, Blocking>,
    pub dc: Output<'static>,
    pub cs: Output<'static>,
    pub rst: Output<'static>,
    pub backlight: Output<'static>,
}

pub fn configure() -> ConfigureResponse {
    rtt_target::rtt_init_defmt!();
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 73744);
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    let spi = Spi::new(
        peripherals.SPI2, // ?
        Config::default()
            .with_frequency(Rate::from_mhz(40))
            .with_mode(Mode::_0),
    )
    .unwrap()
    .with_sck(peripherals.GPIO6)
    .with_mosi(peripherals.GPIO7);

    let rst = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());
    let dc = Output::new(peripherals.GPIO4, Level::Low, OutputConfig::default());
    let cs = Output::new(peripherals.GPIO5, Level::Low, OutputConfig::default());

    let backlight = Output::new(peripherals.GPIO15, Level::Low, OutputConfig::default());
    let wifi = peripherals.WIFI;

    ConfigureResponse {
        wifi,
        spi,
        dc,
        cs,
        rst,
        backlight,
    }
}
