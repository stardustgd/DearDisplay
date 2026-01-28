use axum::http::StatusCode;
use axum::{Router, body::Body, http::Response, routing::get};
use headless_chrome::Browser;
use headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption;
use headless_chrome::protocol::cdp::Target::CreateTarget;

use crate::error::ApiError;

pub fn routes() -> Router {
    Router::new().route("/api/screenshot", get(screenshot_handler))
}

async fn screenshot_handler() -> Result<Response<Body>, ApiError> {
    let browser = Browser::default()?;

    let tab = browser.new_tab_with_options(CreateTarget {
        url: format!("http://192.168.1.199:3000/screen"),
        left: None,
        top: None,
        width: Some(800),
        height: Some(619), // Chromium top bar is 139 pixels (480 + 139 = 619)
        window_state: None,
        browser_context_id: None,
        enable_begin_frame_control: None,
        new_window: Some(true),
        background: None,
        for_tab: None,
        hidden: None,
    })?;

    tab.wait_until_navigated()?;
    tab.wait_for_element("h1.font-semibold.text-3xl")?;

    let element = tab.find_element("html")?;
    element.scroll_into_view()?;

    let box_model = element.get_box_model()?;
    let mut viewport = box_model.margin_viewport();
    viewport.scale = 1.0;

    let screenshot = tab.capture_screenshot(
        CaptureScreenshotFormatOption::Png,
        Some(100),
        Some(viewport),
        true,
    )?;

    tokio::fs::write("screenshot.png", &screenshot).await?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/png")
        .body(Body::from(screenshot))?)
}
