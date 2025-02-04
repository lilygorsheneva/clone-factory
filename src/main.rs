use std::{cell::RefCell, rc::Rc};

use error::{Result, Status};
use game_state::game::Game;
use interface_egui::worldwindow::WorldWindowWidget;
use interface_egui::{
    crafting::CraftingMenu, inventory::inventory, movement::movement, recording::RecorderMenu,
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

pub type GameFn = dyn Fn(&mut Game) -> Result<()>;

struct Application {
    data: &'static Data,
    game: Rc<RefCell<Game>>,
    error: Result<()>,
    command: Option<Box<GameFn>>,
}

impl Application {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let data = Data::get_config();
        Application {
            data: data,
            game: worldgen::start_game(data),
            error: Ok(()),
            command: None,
        }
    }

    pub fn queue_act(&mut self, command: Box<GameFn>) {
        if self.error.is_ok() && self.command.is_none() {
            self.command = Some(command);
        }
    }

    pub fn execute(&mut self) {
        if let Some(cmd) = self.command.as_ref() {
            let mut game = self.game.borrow_mut();
            self.error = cmd(&mut game);
        }
        self.command = None;
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            inventory(self, ctx);

            movement(self, ctx);

            let crafting = CraftingMenu::new(self.game.clone());
            crafting.show(ctx);

            RecorderMenu::show(self, ctx);

            let painter = ui.painter();
            let area = painter.clip_rect();
            {
                let game = self.game.borrow();
                let window = WorldWindowWidget::new(&game);
                let shapes = window.paint(area);
                painter.extend(shapes);
            }

            if let Err(e) = self.error {
                let window = egui::Window::new("Error").show(ctx, |ui| {
                         let text = match e {
            Status::ActionFail(str) => str,
            Status::OutOfBounds => "Some operation returned out of bounds. This should not be player-visible.",
            Status::StateUpdateError =>"Error updating world state.",
            Status::Error(str) =>str,
        };
                    let button = ui.button(text);
                    if button.clicked() {
                        self.error = Ok(())
                    }
                });

            }
        });

        if self.command.is_some() {
            self.execute();
        }
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(Application::new(cc)))),
    )
    .unwrap();
}
