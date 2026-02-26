use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::{Blocking, delay::Delay, gpio::Output, spi::master::Spi};
use mipidsi::{
    Builder, Display,
    interface::SpiInterface,
    models::ST7789,
    options::{ColorInversion, Orientation, Rotation},
};

pub struct CreateDisplayOptions {
    pub spi: Spi<'static, Blocking>,
    pub rst: Output<'static>,
    pub dc: Output<'static>,
    pub cs: Output<'static>,
}

pub fn create_display(
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
    let buffer = {
        static CELL: static_cell::StaticCell<[u8; 512]> = static_cell::StaticCell::new();
        CELL.init([0u8; 512])
    };
    let mut delay = Delay::new();
    let spi_device = ExclusiveDevice::new_no_delay(spi, cs).unwrap();
    let di = SpiInterface::new(spi_device, dc, buffer);
    let display = Builder::new(ST7789, di)
        .reset_pin(rst)
        .invert_colors(ColorInversion::Inverted)
        .init(&mut delay)
        .unwrap();

    let display = {
        let mut display = display;
        display
            .set_orientation(Orientation::new().rotate(Rotation::Deg270))
            .unwrap();
        display
    };

    display
}
