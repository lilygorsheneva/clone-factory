use crate::{
    action::{self, Action},
    direction::{self, AbsoluteDirection},
    game_state::game::Game,
    app::Application,
};

pub fn movement(app: &mut Application, ctx: &egui::Context) {
    let window = egui::SidePanel::right("Controls").show(ctx, |ui| {
        let button = ui.button("West (A)");
        if button.clicked()
            || ui.input(|i| i.key_pressed(egui::Key::ArrowLeft) || i.key_pressed(egui::Key::A))
        {
            app.queue_act(Box::new(|game: &mut Game| {
                game.player_action_and_turn(Action {
                    direction: direction::Direction::Absolute(AbsoluteDirection::W),
                    action: action::SubAction::Move,
                })
            }));
        }
        let button = ui.button("North (W)");
        if button.clicked()
            || ui.input(|i| i.key_pressed(egui::Key::ArrowUp) || i.key_pressed(egui::Key::W))
        {
            app.queue_act(Box::new(|game: &mut Game| {
                game.player_action_and_turn(Action {
                    direction: direction::Direction::Absolute(AbsoluteDirection::N),
                    action: action::SubAction::Move,
                })
            }));
        }

        let button = ui.button("East (D)");
        if button.clicked()
            || ui.input(|i| i.key_pressed(egui::Key::ArrowRight) || i.key_pressed(egui::Key::D))
        {
            app.queue_act(Box::new(|game: &mut Game| {
                game.player_action_and_turn(Action {
                    direction: direction::Direction::Absolute(AbsoluteDirection::E),
                    action: action::SubAction::Move,
                })
            }));
        }

        let button = ui.button("South (S)");
        if button.clicked()
            || ui.input(|i| i.key_pressed(egui::Key::ArrowDown) || i.key_pressed(egui::Key::S))
        {
            app.queue_act(Box::new(|game: &mut Game| {
                game.player_action_and_turn(Action {
                    direction: direction::Direction::Absolute(AbsoluteDirection::S),
                    action: action::SubAction::Move,
                })
            }));
        }

        let button = ui.button("Wait (Space0");
        if button.clicked() || ui.input(|i| i.key_pressed(egui::Key::Space)) {
            app.queue_act(Box::new(|game: &mut Game| {
                game.player_action_and_turn(Action {
                    direction: direction::Direction::Absolute(AbsoluteDirection::S),
                    action: action::SubAction::Wait,
                })
            }));
        }

        let button = ui.button("Take (T)");
        if button.clicked() ||  ui.input(|i| i.key_pressed(egui::Key::T)) {
            app.queue_act(Box::new(|game: &mut Game| {
                game.player_action_and_turn(Action {
                    direction: direction::Direction::Relative(direction::RelativeDirection::F),
                    action: action::SubAction::Take,
                })
            }));
        }
        let button = ui.button("Use Building/Mine (U)");
        if button.clicked()  || ui.input(|i| i.key_pressed(egui::Key::U)) {
            app.queue_act(Box::new(|game: &mut Game| {
                game.player_action_and_turn(Action {
                    direction: direction::Direction::Relative(direction::RelativeDirection::F),
                    action: action::SubAction::ActivateBuilding,
                })
            }));
        }
    });
}
