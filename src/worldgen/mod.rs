use std::{cell::RefCell, rc::Rc};

use crate::game_state::world::FloorTile;
use crate::{buildings::Building, datatypes::Coordinate, game_state::game::Game, static_data::Data};

use crate::engine::update::UpdatableContainer;

pub fn start_game(data: &'static Data) -> Rc<RefCell<Game>>{
    let mut game = Game::new(Coordinate { x: 200, y: 200 }, data);

    game.spawn(&Coordinate { x: 1, y: 1 }).unwrap();


    let ore = 
        data
        .buildings
        .get(&"crystal_deposit".to_string())
        .unwrap();

    game.world
        .buildings
        .mut_set(
            &Coordinate { x: 5, y: 5 },
            &Some(Building { definition: ore }),
        )
        .unwrap();

    for i in 0..31 {
        game.world.floor.mut_set(&Coordinate { x: i, y: i }, &FloorTile::Stone).unwrap();
        game.world.floor.mut_set(&Coordinate { x: 30, y: i }, &FloorTile::Water).unwrap();
        game.world.floor.mut_set(&Coordinate { x: i, y: 30 }, &FloorTile::Water).unwrap();
    }

    Rc::new(RefCell::new(game))
}