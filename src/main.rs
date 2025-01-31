use std::{borrow::Borrow, cell::{Ref, RefCell}, rc::Rc};

use action::Action;
use datatypes::Coordinate;
use direction::RelativeDirection;
use eframe::App;
use game_state::game::Game;
use interface::{menu::MenuTrait, widgets::WorldWindowWidget};
use static_data::Data;

mod action;
mod actor;
mod datatypes;
mod devtools;
mod direction;
mod engine;
mod error;
//mod eventloop;
mod game_state;
mod interface;
mod inventory;
mod static_data;

mod recording;
mod eventqueue;
mod buildings;
mod score;
mod paradox;
mod worldgen;
mod interface_egui;

struct Application {
    data: &'static Data,
    game: Rc<RefCell<Game>>,
}

impl Application {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let data = Data::get_config();
        Application {
            data: data,
            game: worldgen::start_game(data)
        }
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut game = self.game.borrow_mut();

        egui::CentralPanel::default().show(ctx, |ui| {
            let button = ui.button("Clickme!");
            if button.clicked() {
                &game.player_action(Action{direction:direction::Direction::Relative(RelativeDirection::F), action: action::SubAction::Move}).unwrap();
            }
            
            let painter = ui.painter();
            let area = painter.clip_rect();
            let window = WorldWindowWidget::new(&game);
            let shapes = window.paint(area);
            print!("{}\n",shapes.len());
            painter.extend(shapes);
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Ok(Box::new(Application::new(cc)))));
}
