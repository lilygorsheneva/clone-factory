use datatypes::Coordinate;
use inventory::Item;

use game::Game;
use game_state::world::WorldCell;

mod action;
mod actor;
mod static_data;
mod datatypes;
mod game_state;
mod devtools;
mod direction;
mod error;
mod eventloop;
mod game;
mod input;
mod render;
mod inventory;

fn main() {
    let mut terminal = render::init_render();

    let mut game = Game::new(Coordinate { x: 20, y: 10 });
    game.load_gamedata();

    game.spawn(&Coordinate { x: 1, y: 1 }).unwrap();

    let foo = Item::new(0, 1);

    game.world
        .mut_set(
            &Coordinate { x: 10, y: 5 },
            Some(WorldCell {
                actor: None,
                building: None,
                items: [Some(foo)],
            }),
        )
        .unwrap();

    terminal.draw(|frame| render::draw(&game, frame)).unwrap();
    eventloop::main_event_loop(&mut game, &mut terminal);

    render::deinit_render();
}
