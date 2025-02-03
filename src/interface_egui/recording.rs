use crate::{game_state::game::Game, recording::interface::RecordingModule, Application};

pub struct RecorderMenu {}

impl RecorderMenu {
    pub fn show(app: &mut Application, ctx: &egui::Context) {
        let window = egui::Window::new("Recording").show(ctx, |ui| {
            if ui.button("Start recording").clicked() {
                app.queue_act(Box::new(|game: &mut Game| {
                    RecordingModule::init_record(game, 0)
                }));
            }
            if ui.button("Loop").clicked() {
                app.queue_act(Box::new(|game: &mut Game| {
                    RecordingModule::end_record(game, true)
                }));
            }
            if ui.button("Die").clicked() {
                app.queue_act(Box::new(|game: &mut Game| {
                    RecordingModule::end_record(game, false)
                }));
            }
            if ui.button("Replicate").clicked() {
                app.queue_act(Box::new(|game: &mut Game|  RecordingModule::take_item(game)));
            }
        });
    }
}
