use interface::menu::{MenuTrait};

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

fn main() {
    let mut terminal = interface::render::init_render();

    let mut menu = interface::menu::mainmenu::MainMenu::new();
    menu.call(&mut terminal);
    interface::render::deinit_render();
}
