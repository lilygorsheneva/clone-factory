use action::Action;
use datatypes::{Actor, ActorRef, Coordinate};
use world::World;

mod action;
mod datatypes;
mod direction;
mod input;
mod render;
mod world;

fn main() {
    render::init_render();

    let center = Coordinate { x: 5, y: 5 };
    let mut world = World::init(Coordinate { x: 20, y: 10 });

    let mut player_ref = ActorRef{location: Coordinate{x: 1, y:1}};
    world = world.spawn(&player_ref.location, Actor::new()).unwrap();


    render::render(&world, &center);

    loop {
        match input::readinput() {
            Some(input::InputResult::Exit) => break,
            Some(input::InputResult::Redraw) => render::render(&world, &center),
            Some(input::InputResult::Act(act)) => {
                world = action::execute_action(&mut player_ref, act, world);
                render::render(&world, &center);
            },
            _ => {}
        };
    }

    render::deinit_render();
}
