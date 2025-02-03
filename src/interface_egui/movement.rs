use crate::{action::{self, Action}, direction::{self, AbsoluteDirection}, game_state::game::Game, Application};

pub fn movement(app: &mut Application, ctx: &egui::Context) {
    let window = egui::Window::new("Directions").show(ctx, |ui| {
        let button = ui.button("W");
        if button.clicked() || ui.input(|i| i.key_pressed(egui::Key::ArrowLeft)){
            app.queue_act(Box::new(|game: &mut Game|
            game.player_action_and_turn(Action{direction:direction::Direction::Absolute(AbsoluteDirection::W), action: action::SubAction::Move})));
        }
        let button = ui.button("N");
        if button.clicked() || ui.input(|i| i.key_pressed(egui::Key::ArrowUp)){
            app.queue_act(Box::new(|game: &mut Game|
            game.player_action_and_turn(Action{direction:direction::Direction::Absolute(AbsoluteDirection::N), action: action::SubAction::Move})));
        }
        
        let button = ui.button("E");
        if button.clicked() || ui.input(|i| i.key_pressed(egui::Key::ArrowRight)){
            app.queue_act(Box::new(|game: &mut Game|
            game.player_action_and_turn(Action{direction:direction::Direction::Absolute(AbsoluteDirection::E), action: action::SubAction::Move})));
        }
        
        let button = ui.button("S");
        if button.clicked() || ui.input(|i| i.key_pressed(egui::Key::ArrowDown)){
            app.queue_act(Box::new(|game: &mut Game|
            game.player_action_and_turn(Action{direction:direction::Direction::Absolute(AbsoluteDirection::S), action: action::SubAction::Move})));
        }

        let button = ui.button("Take");
        if button.clicked(){
            app.queue_act(Box::new(|game: &mut Game|
            game.player_action_and_turn(Action{direction:direction::Direction::Relative(direction::RelativeDirection::F), action: action::SubAction::Take})));
        }
        let button = ui.button("Interact (building)");
        if button.clicked(){
            app.queue_act(Box::new(|game: &mut Game|
            game.player_action_and_turn(Action{direction:direction::Direction::Relative(direction::RelativeDirection::F), action: action::SubAction::ActivateBuilding})));
        }
    });
}
