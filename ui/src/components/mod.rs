use embedded_graphics::pixelcolor::Rgb565;

pub mod alpha;
pub mod blocks;
pub mod fonts;
pub mod icons;
pub mod schedule;
pub mod temperature;
pub mod utils;

// https://rgbcolorpicker.com/565

pub const BACKGROUND_COLOR: Rgb565 = Rgb565::new(28, 37, 27);
pub const TEXT_COLOR: Rgb565 = Rgb565::new(31, 63, 31);
