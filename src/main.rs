use datatypes::{Actor, Coordinate};
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
    let mut world = World::init(Coordinate { x: 10, y: 10 });
    world.spawn(Coordinate::new(5,5), Actor::new());


    render::render(&world, &center);

    loop {
        match input::readinput() {
            Some(input::InputResult::Exit) => break,
            Some(input::InputResult::Redraw) => render::render(&world, &center),
            _ => {},
        };
    }

    render::deinit_render();
}
