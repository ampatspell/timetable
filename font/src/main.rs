use std::fs::{read_to_string, write};

use headless_chrome::{
    Browser,
    protocol::cdp::{Page::CaptureScreenshotFormatOption::Png, Target::CreateTarget},
};

fn main() {
    let content = read_to_string("src/template.html").expect("Template");
    let script = format!("document.body.innerHTML = `{content}`");

    let browser = Browser::default().unwrap();
    let tab = browser
        .new_tab_with_options(CreateTarget {
            url: String::from("about:blank"),
            left: None,
            top: None,
            width: Some(1024),
            height: Some(300),
            window_state: None,
            browser_context_id: None,
            enable_begin_frame_control: None,
            new_window: Some(true),
            background: None,
            for_tab: None,
            hidden: Some(false),
        })
        .unwrap();

    tab.evaluate(&script, false).unwrap();
    tab.wait_for_element(".done").unwrap();
    let screenshot = tab.capture_screenshot(Png, None, None, true).unwrap();

    write("screenshot.png", screenshot).unwrap();
}
