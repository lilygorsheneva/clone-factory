use actor::{Actor, ActorRef};
use datatypes::Coordinate;
use world::Game;

mod action;
mod datatypes;
mod direction;
mod input;
mod render;
mod world;
mod actor;

fn main() {
    render::init_render();

    let mut game = Game::new(Coordinate { x: 20, y: 10 });

    game.spawn( &Coordinate{x:1, y:1});

    render::render(&game.world, &game.get_player_coords());

    loop {
        match input::readinput() {
            Some(input::InputResult::Exit) => break,
            Some(input::InputResult::Redraw) => render::render(&game.world, &game.get_player_coords()),
            Some(input::InputResult::Act(act)) => {
                action::execute_action(&mut game.actors.player.as_mut().unwrap().actor_ref, act, &mut game.world);
                render::render(&game.world, &game.get_player_coords());
            },
            _ => {}
        };
    }

    render::deinit_render();
}
