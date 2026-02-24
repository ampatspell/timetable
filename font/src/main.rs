use std::{ffi::OsStr, fs::write};

use headless_chrome::{
    Browser, LaunchOptions, protocol::cdp::Page::CaptureScreenshotFormatOption::Png,
};
use pix::{
    chan::{Ch8, Srgb, Straight},
    el::Pix4,
    rgb::Rgb,
};
use png_pong::Decoder;

type Rgba8Raster = pix::Raster<Pix4<Ch8, Rgb, Straight, Srgb>>;

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

pub fn load_raster(png: Vec<u8>) -> Rgba8Raster {
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

pub fn split_raster(raster: Rgba8Raster) {}

fn main() {
    let font_size = 20;
    let png = create_png(font_size);
    let raster = load_raster(png);
}
