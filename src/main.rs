use datatypes::Coordinate;

mod datatypes;
mod render;
mod input;

fn main() {
render::init_render();

let center=  Coordinate {x:5, y:5};
let world = datatypes::World::init(Coordinate{x:10,y:10});

render::render(&world, &center);


loop {
    match input::readinput() {
        Ok(input::InputResult::Exit) => break,
        Ok(input::InputResult::Redraw) => render::render(&world, &center),
        _ => Ok({}),
    };
}

render::deinit_render();

}
