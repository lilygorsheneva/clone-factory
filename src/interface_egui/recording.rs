use crate::{game_state::game::Game, recording::{self, interface::RecordingModule}, Application};

pub struct RecorderMenu {}

impl RecorderMenu {
    pub fn show(app: &mut Application, ctx: &egui::Context) {
        let window = egui::Window::new("Recording").show(ctx, |ui| {
            let mut now_recording = false;
            let mut has_recording = false;
            {
                let recoding_module = &app.game.borrow().recordings;  
                if let Some(rec) = &recoding_module.current_recording {
                    ui.label(format!("Now recording {} steps", rec.len()));
                    now_recording = true;
                }
                has_recording = recoding_module.temp_item.is_some()
            }
            if now_recording {
            if ui.button("End current recording; loop.").clicked() {
                app.queue_act(Box::new(|game: &mut Game| {
                    RecordingModule::end_record(game, true)
                }));
            }
            if ui.button("End current recording; No loop.").clicked() {
                app.queue_act(Box::new(|game: &mut Game| {
                    RecordingModule::end_record(game, false)
                }));
            }
        }
        if has_recording {
            if ui.button("Add recording to inventory").clicked() {
                app.queue_act(Box::new(|game: &mut Game|  RecordingModule::take_item(game)));
            }}
        });
        
    }
}
