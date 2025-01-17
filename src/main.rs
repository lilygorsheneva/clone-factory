use datatypes::Coordinate;
use engine::update::Updatable;
use inventory::Item;

use game_state::game::Game;
use static_data::StaticData;

mod action;
mod actor;
mod datatypes;
mod devtools;
mod direction;
mod engine;
mod error;
mod eventloop;
mod game_state;
mod interface;
mod inventory;
mod static_data;

fn main() {
    let mut app = eventloop::Application::new();
    app.main_menu();
}
