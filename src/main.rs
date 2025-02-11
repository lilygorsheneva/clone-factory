mod action;
mod actor;
mod datatypes;
mod devtools;
mod direction;
mod engine;
mod error;
mod game_state;
mod inventory;
mod static_data;

mod buildings;
mod eventqueue;
mod interface_egui;
mod paradox;
mod recording;
mod score;
mod worldgen;
mod app;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Clone Factory",
        native_options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(app::Application::new(cc)))
        }),
    )
    .unwrap();
}
