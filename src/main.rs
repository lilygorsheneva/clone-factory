use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use action::Action;
use datatypes::Coordinate;
use direction::{AbsoluteDirection, RelativeDirection};
use eframe::App;
use game_state::game::Game;
use interface::widgets::WorldWindowWidget;
use interface_egui::{
    crafting::{self, CraftingMenu},
    movement::movement,
};
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

mod buildings;
mod eventqueue;
mod interface_egui;
mod paradox;
mod recording;
mod score;
mod worldgen;

struct Application {
    data: &'static Data,
    game: Rc<RefCell<Game>>,
}

impl Application {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let data = Data::get_config();
        Application {
            data: data,
            game: worldgen::start_game(data),
        }
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            movement(self.game.clone(), ctx);
            let crafting = CraftingMenu::new(self.game.clone());
            crafting.show(ctx);

            let painter = ui.painter();
            let area = painter.clip_rect();
            {
                let game = self.game.borrow();
                let window = WorldWindowWidget::new(&game);
                let shapes = window.paint(area);
                painter.extend(shapes);
            }
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(Application::new(cc)))),
    ).unwrap();
}
