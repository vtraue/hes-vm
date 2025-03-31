use std::fs;

use eframe::{egui, run_native};
use gui::HesApp;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1800.0, 1200.0])
            .with_min_inner_size([660.0, 440.0]), //.with_icon(
        //    eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon.png")[..])
        //        .expect("Failed to load icon"),
        ..Default::default()
    };

    let path = "gen.wasm";
    run_native(
        "hes-vm",
        native_options,
        Box::new(|cc| Ok(Box::new(HesApp::new(cc, path)))),
    )
}
