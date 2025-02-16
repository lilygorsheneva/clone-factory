use egui::Context;

use crate::{error::Status,     app::Application};

pub fn show(app: &mut Application, ctx: &Context) {
    if let Err(e) = app.error {
        let window = egui::Modal::new(egui::Id::new("Error")).show(ctx, |ui| {
            let mut recoverable = false;
            let text = match e {
                Status::ActionFail(str) =>{recoverable = true; str},
                Status::OutOfBounds => {
                    "Some operation returned out of bounds. This should not be player-visible."
                }
                Status::StateUpdateError => "Error updating world state.",
                Status::Error(str) => str,
            };
            let button = ui.button(text);
            if recoverable && (button.clicked() || ui.input(|i| i.key_pressed(egui::Key::Escape))) {
                app.error = Ok(())
            }
        });
    }
}
