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
    let mut terminal =render::init_render();

    let mut game = Game::new(Coordinate { x: 20, y: 10 });

    game.spawn(&Coordinate { x: 1, y: 1 }).unwrap();

    let foo = Item::new(0, 1);

    game.world.mut_set(
        &Coordinate { x: 10, y: 5 },
        Some(WorldCell {
            actor: None,
            building: None,
            items: [Some(foo)],
        }),
    ).unwrap();

    terminal.draw(|frame| render::draw(&game, frame)).unwrap();

    loop {
        match input::readinput() {
            Some(input::InputResult::Exit) => break,
            Some(input::InputResult::Redraw) => {
                terminal.draw(|frame| render::draw(&game, frame)).unwrap();
            }
            Some(input::InputResult::Act(act)) => {
                game.player_action(act).unwrap();
                game.do_npc_turns().unwrap();
                terminal.draw(|frame| render::draw(&game, frame)).unwrap();
            }
            Some(input::InputResult::Record) => match game.current_recording {
                Some(_) => game.end_record().unwrap(),
                None => game.init_record().unwrap(),
            },
            _ => {}
        };
    }

    render::deinit_render();
}
