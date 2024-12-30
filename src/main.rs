use datatypes::{Coordinate, Item};
use world::{WorldCell};
use game::Game;

mod action;
mod actor;
mod datatypes;
mod direction;
mod input;
mod render;
mod world;
mod game;
mod db;

fn main() {
    render::init_render();

    let mut game = Game::new(Coordinate { x: 20, y: 10 });

    game.spawn(&Coordinate { x: 1, y: 1 });

    let foo = Item::new("Foo".to_string(), 1);

    game.world.set(
        &Coordinate { x: 10, y: 5 },
        Some(WorldCell {
            actor: None,
            building: None,
            items: [Some(foo)],
        }),
    );

    render::render(&game.world, &game.get_player_coords());

    loop {
        match input::readinput() {
            Some(input::InputResult::Exit) => break,
            Some(input::InputResult::Redraw) => {
                render::render(&game.world, &game.get_player_coords())
            }
            Some(input::InputResult::Act(act)) => {
                action::execute_action(
                    game.actors.get_mut_player(),
                    act,
                    &mut game.world,
                );
                render::render(&game.world, &game.get_player_coords());
            }
            _ => {}
        };
    }

    render::deinit_render();
}
