use crate::{action::{self, Action}, direction::{Direction, RelativeDirection}, game_state::game::Game, Application};

pub fn inventory(app: &mut Application, ctx: &egui::Context) {
    let window = egui::Window::new("Inventory").show(ctx, |ui| {
        let button = ui.button("Use 1");
        if button.clicked() {
            app.queue_act(Box::new(|game: &mut Game|
            game.player_action_and_turn(Action{direction: Direction::Relative(RelativeDirection::F), action: action::SubAction::Use(0)})));
        }
        let button = ui.button("Drop 1");
        if button.clicked() {
            app.queue_act(Box::new(|game: &mut Game|
            game.player_action_and_turn(Action{direction: Direction::Relative(RelativeDirection::F), action: action::SubAction::Drop(0)})));
        }
    });}

