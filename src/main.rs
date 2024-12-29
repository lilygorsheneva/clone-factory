use datatypes::Coordinate;

mod datatypes;
mod render;

fn main() {
//render::init_render();

let center=  Coordinate {x:5, y:5};
let world = datatypes::World::init(Coordinate{x:10,y:10});

render::render(world, center);

//render::deinit_render();

}
