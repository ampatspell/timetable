use std::{ffi::OsStr, fs::write};

use headless_chrome::{
    Browser, LaunchOptions, protocol::cdp::Page::CaptureScreenshotFormatOption::Png,
};

fn main() {
    let font_size = 20;

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

    write("screenshot.png", screenshot).unwrap();
}
