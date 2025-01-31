use crate::{action::{self, Action}, direction::{self, AbsoluteDirection}, game_state::game::Game};

pub fn movement(game: &mut Game, ctx: &egui::Context) {
    let window = egui::Window::new("Directions").show(ctx, |ui| {
        let button = ui.button("W");
        if button.clicked() {
            &game.player_action(Action{direction:direction::Direction::Absolute(AbsoluteDirection::W), action: action::SubAction::Move}).unwrap();
        }
        let button = ui.button("N");
        if button.clicked() {
            &game.player_action(Action{direction:direction::Direction::Absolute(AbsoluteDirection::N), action: action::SubAction::Move}).unwrap();
        }
        
        let button = ui.button("E");
        if button.clicked() {
            &game.player_action(Action{direction:direction::Direction::Absolute(AbsoluteDirection::E), action: action::SubAction::Move}).unwrap();
        }
        
        let button = ui.button("S");
        if button.clicked() {
            &game.player_action(Action{direction:direction::Direction::Absolute(AbsoluteDirection::S), action: action::SubAction::Move}).unwrap();
        }
    });
}