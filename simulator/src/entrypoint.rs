use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::{Dimensions, DrawTarget},
};
use embedded_graphics_simulator::{SimulatorDisplay, SimulatorEvent, Window};
use no_std_strings::str128;
use ui::{
    draw::draw_content,
    payload::{Payload, Temperature, Tram, Weather, Wind},
};

pub fn draw<D>(display: &mut D) -> ()
where
    D: DrawTarget<Color = Rgb565> + Dimensions,
{
    // let mut canvas = CCanvasAt::<Rgb565, 280, 240>::new(Point { x: 0, y: 0 });
    // canvas.clear(Rgb565::new(30, 30, 30));

    // let line_style = PrimitiveStyle::with_stroke(Rgb565::RED, 1);
    // let text_style = MonoTextStyle::new(&FONT_6X9, Rgb565::GREEN);
    // Circle::new(Point::new(72, 8), 48)
    //     .into_styled(line_style)
    //     .draw(&mut canvas);
    // Line::new(Point::new(48, 16), Point::new(8, 16))
    //     .into_styled(line_style)
    //     .draw(&mut canvas);
    // Line::new(Point::new(48, 16), Point::new(64, 32))
    //     .into_styled(line_style)
    //     .draw(&mut canvas);
    // Rectangle::new(Point::new(79, 15), Size::new(34, 34))
    //     .into_styled(line_style)
    //     .draw(&mut canvas);
    // Text::new("Hello World!", Point::new(5, 5), text_style)
    //     .draw(&mut canvas)
    //     .ok();

    // canvas.draw(display).ok();

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
                time: str128::from("03:55"),
                adjustment: -103,
            },
        )
            .into(),
    };

    draw_content(display, payload);
}

pub fn main_loop(display: &mut SimulatorDisplay<Rgb565>, window: &mut Window) -> () {
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
