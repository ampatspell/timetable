use embedded_graphics::pixelcolor::Rgb565;

pub mod schedule;
pub mod temperature;
pub mod utils;

// https://rgbcolorpicker.com/565

pub const BACKGROUND_COLOR: Rgb565 = Rgb565::new(7, 13, 7);
pub const TEXT_COLOR: Rgb565 = Rgb565::new(30, 61, 30);
