use std::{cell::RefCell, rc::Rc};

use crate::actor::Actor;
use crate::eventqueue::ActorEvent;
use crate::game_state::world::FloorTile;
use crate::{
    buildings::Building, datatypes::Coordinate, game_state::game::Game, static_data::Data,
};
use crate::{devtools, game_state, recording};

use crate::engine::update::UpdatableContainer;

pub fn start_game(data: &'static Data) -> Rc<RefCell<Game>> {
    let mut game = Game::new(Coordinate { x: 60, y: 60 }, data);

    game.spawn(&Coordinate { x: 1, y: 1 }).unwrap();

    let ore = data.buildings.get(&"crystal_deposit".to_string()).unwrap();

    let oresites = [(1, 1), (1, 2), (2, 1), (2, 2)];

    for i in oresites {
        game.world
            .buildings
            .mut_set(
                &Coordinate { x: i.0, y: i.1 },
                &Some(Building { definition: ore }),
            )
            .unwrap();
    }

    for i in 16..45 {
        game.world
            .floor
            .mut_set(&Coordinate { x: i, y: i }, &FloorTile::Stone)
            .unwrap();
        game.world
            .floor
            .mut_set(&Coordinate { x: 15, y: i }, &FloorTile::Water)
            .unwrap();
        game.world
            .floor
            .mut_set(&Coordinate { x: 45, y: i }, &FloorTile::Water)
            .unwrap();
        game.world
            .floor
            .mut_set(&Coordinate { x: i, y: 15 }, &FloorTile::Water)
            .unwrap();
        game.world
            .floor
            .mut_set(&Coordinate { x: i, y: 45 }, &FloorTile::Water)
            .unwrap();
    }

    let recording = devtools::make_sample_recording();
    let foedescriptor = data.actors.get("foe").unwrap();
    let recording_id = game
        .recordings
        .load_recording(devtools::make_sample_recording());

    let foesites = [(16, 16), (44, 16), (16, 44), (44, 44)];

    for i in foesites {
        let id = game.world.actors.mut_get_next_id();
        let foeactor = Actor::from_recording(foedescriptor, id, &recording);
        game.world
            .actors
            .mut_set(&Coordinate { x: i.0, y: i.1 }, &Some(foeactor))
            .unwrap();
        game.event_queue.next_turn.push_back(ActorEvent {
            actor: id,
            recording: recording_id,
            recording_idx: 0,
        });
    }

    Rc::new(RefCell::new(game))
}
