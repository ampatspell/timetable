use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics_simulator::{SimulatorDisplay, SimulatorEvent, Window};
use no_std_strings::str128;
use ui::{
    Display,
    draw::{draw_content, draw_first_frame},
    payload::{Payload, Temperature, Tram, Weather, Wind},
};

pub fn draw(display: &mut impl Display) -> () {
    let payload = Payload {
        weather: Weather {
            temperature: Temperature {
                value: -5.7,
                description: str128::from("Snow grains falling."),
            },
            wind: Wind {
                speed: 12.1,
                direction: 37,
            },
        },
        trams: (
            Tram {
                time: str128::from("03:55"),
                adjustment: -103,
            },
            Tram {
                time: str128::from("04:21"),
                adjustment: 0,
            },
        )
            .into(),
    };

    draw_content(display, payload);
}

pub fn main_loop(display: &mut SimulatorDisplay<Rgb565>, window: &mut Window) -> () {
    draw_first_frame(display);
    loop {
        draw(display);
        window.update(display);

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
