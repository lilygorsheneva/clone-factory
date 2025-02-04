use crate::error::Result;
use crate::{
    action::{self, Action},
    direction::{Direction, RelativeDirection},
    game_state::game::Game,
    Application,
};

pub fn inventory(app: &mut Application, ctx: &egui::Context) -> Result<()> {
    let inventory = app.game.borrow().get_player_actor()?.inventory;

    let window = egui::TopBottomPanel::bottom("Inventory").show(ctx, |ui| {
        let items = inventory.get_items();
        ui.horizontal(|ui| {
        for i in 0..items.len() {
            ui.vertical(|ui| {
                ui.label(format!("{}", i + 1));

                if let Some(item) = items[i] {
                    let name = &item.definition.text.name;
                    ui.label(name);
                    let button = ui.button("Use");
                    if button.clicked() {
                        app.queue_act(Box::new(|game: &mut Game| {
                            game.player_action_and_turn(Action {
                                direction: Direction::Relative(RelativeDirection::F),
                                action: action::SubAction::Use(0),
                            })
                        }));
                    }
                    let button = ui.button("Drop");
                    if button.clicked() {
                        app.queue_act(Box::new(|game: &mut Game| {
                            game.player_action_and_turn(Action {
                                direction: Direction::Relative(RelativeDirection::F),
                                action: action::SubAction::Drop(0),
                            })
                        }));
                    }
                }
            });
        }});
    });
    Ok(())
}
