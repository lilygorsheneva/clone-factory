use std::{cell::RefCell, rc::Rc};
use crate::error::Result;
use crate::interface_egui::info;
use crate::{game_state::game::Game, static_data::Data, worldgen};
use crate::interface_egui::{self, crafting::CraftingMenu, inventory, movement, recording::RecorderMenu, worldwindow::WorldWindowWidget};


pub type GameFn = dyn Fn(&mut Game) -> Result<()>;

pub struct Application {
    pub data: &'static Data,
    pub game: Rc<RefCell<Game>>,
    pub error: Result<()>,
    pub command: Option<Box<GameFn>>,
}

impl Application {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
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
            
            inventory::inventory(self, ctx);

            movement::movement(self, ctx);
            info::show(self, ctx);

            let crafting = CraftingMenu::new(self.game.clone());
            crafting.show(self, ctx);

            RecorderMenu::show(self, ctx);

            let painter = ui.painter();
            let area = painter.clip_rect();
            {
                let game = self.game.borrow();
                let window = WorldWindowWidget::new(&game);
                let shapes = window.paint(ctx ,area);
                painter.extend(shapes);
            }

            interface_egui::error::show(self, ctx);
        });

        if self.command.is_some() {
            self.execute();
        }
    }
}