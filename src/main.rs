// src/main.rs
pub mod window;
pub mod data; 

use eframe;
use data::Wawa;

fn create_window() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.,768.])
            .with_min_inner_size([640.,480.])
            .with_icon(
                // NOTE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon.png")[..])
                    .expect("Failed to load icon"),
            ),
        centered: true, 
        ..Default::default()
    };

    // Properly using a closure for run_native
    eframe::run_native(
        "Click the Idle", 
        native_options, 
        Box::new(|_cc| {Ok(Box::new(Wawa::new()))})
    )
}

fn main() -> eframe::Result {
    create_window()
}