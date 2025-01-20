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

mod recording;

fn main() {
    let mut terminal = interface::widgets::init_render();

    let mut menu = interface::menu::mainmenu::MainMenu::new();
    menu.enter_menu(&mut terminal);
    interface::widgets::deinit_render();
}
