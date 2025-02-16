use egui::Ui;

use crate::{app::Application, game_state::world::WorldCell, score::Score, static_data::ObjectDescriptor};

fn object_info(ui: &mut Ui, object: &'static ObjectDescriptor) {
    ui.vertical(|ui| {
        ui.label(&object.text.name);
        ui.label(&object.text.description);
    });
}

fn cell_info(ui: &mut Ui, cell: &WorldCell) {
    ui.label(format!("Local Paradox level: {}", cell.paradox.0));
    if let Some(a) = cell.actor {
        object_info(ui, a.descriptor);
    }

    if let Some(b) = cell.building {
        object_info(ui, b.definition);
    }
    if let Some(i) = cell.items[0] {
        object_info(ui, i.definition);
    }
}

fn score(ui: &mut Ui, score: &Score) {
    ui.label(format!("Score: {}", score.score));
    ui.label(format!("Turn: {}", score.turn));
}

pub fn show(app: &mut Application, ctx: &egui::Context) {
    let game = app.game.borrow();
    let cell = game.world.get_cell(&game.get_player_coords().unwrap()).unwrap();
    let window = egui::SidePanel::left("Info").show(ctx, |ui| {
        score(ui, &game.score);
        cell_info(ui, &cell);
    });
}
