#![allow(dead_code)]

mod app;
mod domain;
mod execution;
mod notebook;
mod reactive;
mod runtime;
mod schematic;
mod simulation;
mod ui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1100.0, 760.0]),
        run_and_return: false,
        ..Default::default()
    };

    eframe::run_native(
        "Tupan — Component Canvas",
        options,
        Box::new(|cc| Ok(Box::new(app::TupanApp::new(cc)))),
    )
}
