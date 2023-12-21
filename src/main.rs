#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
pub mod backends;
pub mod compiler;
pub mod entities;
pub mod gui;
pub mod import;
pub mod ir;
pub mod utils;

use std::{fs::File, sync::Arc};

use eframe::NativeOptions;
use gui::MyApp;

pub fn main() -> Result<(), eframe::Error> {
    let file = File::create("debug.log").unwrap();
    tracing_subscriber::fmt().with_writer(Arc::new(file)).init();
    eframe::run_native(
        "Factorio Verify",
        NativeOptions::default(),
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<MyApp>::default()
        }),
    )
}
