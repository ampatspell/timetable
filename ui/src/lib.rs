#![no_std]

use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::{Dimensions, DrawTarget},
};

pub mod components;
pub mod payload;
pub mod ui;

pub trait Display: DrawTarget<Color = Rgb565> + Dimensions {}
impl<T: DrawTarget<Color = Rgb565> + Dimensions> Display for T {}
