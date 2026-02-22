use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics_simulator::{SimulatorDisplay, SimulatorEvent, Window};
use ui::draw::UI;

pub fn main_loop(display: &mut SimulatorDisplay<Rgb565>, window: &mut Window) -> () {
    let mut ui = UI::new();
    ui.prepare(display);
    loop {
        ui.draw(display);
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => return (),
                SimulatorEvent::KeyDown { keycode, .. } => {
                    println!("KeyDown {}", keycode);
                }
                SimulatorEvent::MouseButtonUp { point, .. } => {
                    println!("MouseUp {}", point);
                }
                _ => {}
            }
        }
    }
}
