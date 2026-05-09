use eframe::NativeOptions;
use egui::ViewportBuilder;

use crate::views::App;
mod views;
fn main() -> Result<(), eframe::Error> {
    let native_options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_resizable(false)
            .with_inner_size([500.0, 650.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Calculator",
        native_options,
        Box::new(|cc| {
            let app = App::default();
            App::set_styles(&cc.egui_ctx);
            Ok(Box::new(app))
        }),
    )
}
