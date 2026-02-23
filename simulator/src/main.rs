use crate::entrypoint::main_loop;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

mod entrypoint;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(350, 350));
    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = Window::new("Simulator", &output_settings);

    main_loop(&mut display, &mut window);

    Ok(())
}
