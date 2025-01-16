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
    let mut terminal = interface::render::init_render();

    let data = StaticData::get_config();
    let mut game = Game::new(Coordinate { x: 20, y: 10 }, &data);

    game.spawn(&Coordinate { x: 1, y: 1 }).unwrap();

    let item_def = data.items.get(&"raw_crystal".to_string()).unwrap();
    let foo = Item::new(item_def, 1);

    game.world
        .items
        .mut_set(&Coordinate { x: 10, y: 5 }, &[Some(foo)])
        .unwrap();

    terminal
        .draw(|frame| interface::render::draw(&game, frame))
        .unwrap();
    eventloop::main_event_loop(&mut game, &mut terminal);

    interface::render::deinit_render();
}
