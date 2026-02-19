use embedded_canvas::CCanvasAt;
use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_6X9},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, Rectangle},
    text::Text,
};
use embedded_graphics_simulator::{SimulatorDisplay, SimulatorEvent, Window};

pub fn draw(display: &mut SimulatorDisplay<Rgb565>) -> () {
    let mut canvas = CCanvasAt::<Rgb565, 280, 240>::new(Point { x: 0, y: 0 });
    canvas.clear(Rgb565::new(30, 30, 30));

    let line_style = PrimitiveStyle::with_stroke(Rgb565::RED, 1);
    let text_style = MonoTextStyle::new(&FONT_6X9, Rgb565::GREEN);
    Circle::new(Point::new(72, 8), 48)
        .into_styled(line_style)
        .draw(&mut canvas);
    Line::new(Point::new(48, 16), Point::new(8, 16))
        .into_styled(line_style)
        .draw(&mut canvas);
    Line::new(Point::new(48, 16), Point::new(64, 32))
        .into_styled(line_style)
        .draw(&mut canvas);
    Rectangle::new(Point::new(79, 15), Size::new(34, 34))
        .into_styled(line_style)
        .draw(&mut canvas);
    Text::new("Hello World!", Point::new(5, 5), text_style)
        .draw(&mut canvas)
        .ok();

    canvas.draw(display);
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
