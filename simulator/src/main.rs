use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_6X9},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, Rectangle},
    text::Text,
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

fn draw(display: &mut SimulatorDisplay<Rgb565>) -> () {
    let line_style = PrimitiveStyle::with_stroke(Rgb565::RED, 1);
    let text_style = MonoTextStyle::new(&FONT_6X9, Rgb565::GREEN);
    Circle::new(Point::new(72, 8), 48)
        .into_styled(line_style)
        .draw(display);
    Line::new(Point::new(48, 16), Point::new(8, 16))
        .into_styled(line_style)
        .draw(display);
    Line::new(Point::new(48, 16), Point::new(64, 32))
        .into_styled(line_style)
        .draw(display);
    Rectangle::new(Point::new(79, 15), Size::new(34, 34))
        .into_styled(line_style)
        .draw(display);
    Text::new("Hello World!", Point::new(5, 5), text_style)
        .draw(display)
        .ok();
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(280, 240));
    let output_settings = OutputSettingsBuilder::new().scale(3).build();
    let mut window = Window::new("Simulator", &output_settings);

    'running: loop {
        draw(&mut display);
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
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

    Ok(())
}
