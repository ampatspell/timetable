use std::{
    ffi::OsStr,
    fs::{create_dir, write},
};

use headless_chrome::{
    Browser, LaunchOptions, protocol::cdp::Page::CaptureScreenshotFormatOption::Png,
};
use pix::{
    Raster,
    chan::{Ch8, Channel, Srgb, Straight},
    el::Pixel,
    rgb::{Rgb, SRgba8},
};
use png_pong::{Decoder, Encoder, PngRaster};

type Raster8 = Raster<pix::el::Pix3<Ch8, Rgb, Straight, Srgb>>;

pub fn create_png(font_size: u16) -> Vec<u8> {
    let args = vec![OsStr::new(
        "--disable-features=HttpsUpgrades,HttpsFirstBalancedModeAutoEnable",
    )];

    let browser = Browser::new(
        LaunchOptions::default_builder()
            .headless(true)
            .args(args)
            .window_size(Some((1920, 600)))
            .build()
            .unwrap(),
    )
    .unwrap();

    let tab = browser.new_tab().unwrap();
    tab.navigate_to(
        format!("http://timetable.app.amateurinmotion.com/font?font-size={font_size}").as_str(),
    )
    .unwrap()
    .wait_for_element(".done")
    .unwrap();

    let screenshot = tab.capture_screenshot(Png, None, None, true).unwrap();
    write("screenshot.png", &screenshot).unwrap();

    screenshot
}

pub fn load_raster(png: Vec<u8>) -> Raster8 {
    let arr = png.as_slice();
    let decoder = Decoder::new(arr).unwrap();
    let step = decoder.into_steps().last().unwrap().unwrap();
    let raster = step.raster;
    let rgb8 = match raster {
        png_pong::PngRaster::Rgb8(raster) => Some(raster),
        _ => None,
    }
    .unwrap();

    rgb8
}

static OUT: &str = "out";

pub fn prepare_write_glyphs() {
    create_dir(OUT).ok();
}

pub fn write_glyph(output: Raster<SRgba8>, index: u16) {
    let raster = PngRaster::Rgba8(output);
    let mut data = Vec::new();
    let mut encoder = Encoder::new(&mut data).into_step_enc();
    let step = png_pong::Step { raster, delay: 0 };
    encoder.encode(&step).unwrap();
    let path = format!("{OUT}/{index}.png");
    std::fs::write(path, data).unwrap();
}

pub fn save_glyph(raster: &Raster8, index: u16, ox: u16, oy: u16, width: u16, height: u16) {
    prepare_write_glyphs();

    let mut output: Raster<SRgba8> = Raster::with_clear(width as u32, height as u32);
    for (y, row) in output.rows_mut(()).enumerate() {
        for (x, pixel) in row.iter_mut().enumerate() {
            let input = raster.pixel(ox as i32 + x as i32, oy as i32 + y as i32);
            let channels = input.channels();
            let value = (255. * channels[0].to_f32()) as u8;
            *pixel = SRgba8::new(value, value, value, 255);
        }
    }
    write_glyph(output, index);
}

pub fn split_raster(raster: Raster8, def: Definition, glyphs: u16) {
    let ox = 25;
    let y = 25;
    let width = def.width;
    let height = def.height;
    for index in 0..glyphs {
        let x = ox + (index * (def.width + def.padding));
        save_glyph(&raster, index, x, y, width, height);
    }
}

pub struct Definition {
    pub font_size: u16,
    pub width: u16,
    pub height: u16,
    pub padding: u16,
}

fn main() {
    let glyphs: u16 = 86;
    let definition = Definition {
        font_size: 20,
        width: 10,
        height: 20,
        padding: 0,
    };
    let png = create_png(definition.font_size);
    let raster = load_raster(png);
    split_raster(raster, definition, glyphs);
}
