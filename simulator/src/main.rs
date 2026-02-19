use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

use crate::entrypoint::main_loop;

mod entrypoint;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(280, 240));
    let output_settings = OutputSettingsBuilder::new().scale(3).build();
    let mut window = Window::new("Simulator", &output_settings);

    main_loop(&mut display, &mut window);

    Ok(())
}
