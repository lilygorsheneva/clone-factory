use datatypes::{Coordinate, Item};
use game::Game;
use world::WorldCell;

mod action;
mod actor;
mod datatypes;
mod db;
mod devtools;
mod direction;
mod game;
mod input;
mod render;
mod world;
mod error;

fn main() {
    render::init_render();

    let mut game = Game::new(Coordinate { x: 20, y: 10 });

    game.spawn(&Coordinate { x: 1, y: 1 });

    let foo = Item::new(0, 1);

    game.world.set(
        &Coordinate { x: 10, y: 5 },
        Some(WorldCell {
            actor: None,
            building: None,
            items: [Some(foo)],
        }),
    );

    render::renderworld(&game.world, &game.get_player_coords()).unwrap();

    loop {
        match input::readinput() {
            Some(input::InputResult::Exit) => break,
            Some(input::InputResult::Redraw) => {
                render::renderworld(&game.world, &game.get_player_coords()).unwrap();
            }
            Some(input::InputResult::Act(act)) => {
                game.player_action(act);
                game.do_npc_turns();
                render::renderworld(&game.world, &game.get_player_coords()).unwrap();
            }
            Some(input::InputResult::Record) => match game.current_recording {
                Some(_) => game.end_record(),
                None => game.init_record(),
            },
            _ => {}
        };
    }

    render::deinit_render();
}
