mod app;
mod net;
mod render;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "littleBrowser",
        native_options,
        Box::new(|cc| Ok(Box::new(app::BrowserApp::new(cc)))),
    )
}

