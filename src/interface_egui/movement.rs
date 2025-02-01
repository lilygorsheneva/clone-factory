use std::{cell::RefCell, rc::Rc};

use crate::{action::{self, Action}, direction::{self, AbsoluteDirection}, game_state::game::Game};

pub fn movement(game:  Rc<RefCell<Game>>, ctx: &egui::Context) {
    let mut game = game.borrow_mut();
    let window = egui::Window::new("Directions").show(ctx, |ui| {
        let button = ui.button("W");
        if button.clicked() || ui.input(|i| i.key_pressed(egui::Key::ArrowLeft)){
            &game.player_action_and_turn(Action{direction:direction::Direction::Absolute(AbsoluteDirection::W), action: action::SubAction::Move}).unwrap();
        }
        let button = ui.button("N");
        if button.clicked() || ui.input(|i| i.key_pressed(egui::Key::ArrowUp)){
            &game.player_action_and_turn(Action{direction:direction::Direction::Absolute(AbsoluteDirection::N), action: action::SubAction::Move}).unwrap();
        }
        
        let button = ui.button("E");
        if button.clicked() || ui.input(|i| i.key_pressed(egui::Key::ArrowRight)){
            &game.player_action_and_turn(Action{direction:direction::Direction::Absolute(AbsoluteDirection::E), action: action::SubAction::Move}).unwrap();
        }
        
        let button = ui.button("S");
        if button.clicked() || ui.input(|i| i.key_pressed(egui::Key::ArrowDown)){
            &game.player_action_and_turn(Action{direction:direction::Direction::Absolute(AbsoluteDirection::S), action: action::SubAction::Move}).unwrap();
        }

        let button = ui.button("Take");
        if button.clicked(){
            &game.player_action_and_turn(Action{direction:direction::Direction::Relative(direction::RelativeDirection::F), action: action::SubAction::Take}).unwrap();
        }
        let button = ui.button("Interact (building)");
        if button.clicked(){
            &game.player_action_and_turn(Action{direction:direction::Direction::Relative(direction::RelativeDirection::F), action: action::SubAction::ActivateBuilding}).unwrap();
        }
    });
}
