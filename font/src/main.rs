use std::{ffi::OsStr, fs::write};

use headless_chrome::{
    Browser, LaunchOptions, protocol::cdp::Page::CaptureScreenshotFormatOption::Png,
};
use pix::{
    Raster,
    chan::{Ch8, Srgb, Straight},
    el::{Pix4, Pixel},
    rgb::{Rgb, Rgba8, SRgb8, SRgba8},
};
use png_pong::{Decoder, Encoder};

type Raster8 = pix::Raster<Pix4<Ch8, Rgb, Straight, Srgb>>;

pub fn create_png(font_size: u16) -> Vec<u8> {
    let args = vec![OsStr::new(
        "--disable-features=HttpsUpgrades,HttpsFirstBalancedModeAutoEnable",
    )];

    let browser = Browser::new(
        LaunchOptions::default_builder()
            .headless(false)
            .args(args)
            .window_size(Some((1024, 600)))
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
    let rgba8 = match raster {
        png_pong::PngRaster::Rgba8(raster) => Some(raster),
        _ => None,
    }
    .unwrap();

    rgba8
}

pub fn save_glyph(raster: &Raster8, ox: u16, oy: u16, width: u16, height: u16) {
    let mut output = Raster::<Rgba8>::with_clear(width as u32, height as u32);
    for x in 0..width {
        for y in 0..height {
            let pixel = raster.pixel((ox + x) as i32, (oy + y) as i32);
            let alpha = pixel.alpha();
            let out = output.pixel_mut(x as i32, y as i32);
        }
    }
}

pub fn split_raster(raster: Raster8, def: Definition) {
    let ox = 25;
    let y = 25;
    let width = def.width;
    let height = def.height;
    let glyphs: u16 = 10 + (2 * 38);
    for index in 0..glyphs {
        let x = ox + (index * (def.width + def.padding));
        save_glyph(&raster, x, y, width, height);
    }
}

pub struct Definition {
    pub font_size: u16,
    pub width: u16,
    pub height: u16,
    pub padding: u16,
}

fn main() {
    let definition = Definition {
        font_size: 20,
        width: 10,
        height: 20,
        padding: 0,
    };
    let png = create_png(definition.font_size);
    let raster = load_raster(png);
    split_raster(raster, definition);
}
