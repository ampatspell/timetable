use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics_simulator::{SimulatorDisplay, SimulatorEvent, Window};
use no_std_strings::str12;
use ui::ui::UI;

pub fn main_loop(display: &mut SimulatorDisplay<Rgb565>, window: &mut Window) -> () {
    let mut ui = UI::new();
    ui.prepare(display);
    loop {
        ui.on_time(display, str12::from("00:00:00"));
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
