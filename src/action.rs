use crate::actor::{self, Actor, ActorRef};
use crate::datatypes::{Coordinate, Item};
use crate::direction::{AbsoluteDirection, Direction};
use crate::game::Game;
use crate::world::WorldCell;

pub struct Action {
    pub direction: Direction,
    pub action: SubAction,
}

pub enum SubAction {
    Move,
    Take,
    // Drop,
    Use(usize),
    // Craft(String),
    // Record,
    // EndRecording,
}

pub fn execute_action(actor_ref: ActorRef, action: Action, game: &mut Game) {
    let orientation = actor_ref.orientation.rotate(&action.direction);

    match action.action {
        SubAction::Move => execute_move(actor_ref.location, orientation, game),
        SubAction::Take => execute_take(actor_ref.location, orientation, game),
        SubAction::Use(i) => execute_use_cloner(i, actor_ref.location, orientation, game),
        // _ => world,
    }
}

fn execute_move(
    location: Coordinate,
    orientation: AbsoluteDirection,
    game: &mut Game,
) {
    let offsets = vec![Coordinate { x: 0, y: 0 }, Coordinate { x: 0, y: 1 }];
    let cells = game.world.getslice(location, orientation, &offsets);

    let src = cells[0].unwrap();
    let dest = cells[1];

    if dest.is_none() || dest.unwrap().actor.is_some() {
        // fail
        return;
    } else {
        let mut new_actor = src.actor.clone().unwrap();
        let actor_ref: &mut ActorRef = game.actors.get_mut_actor(new_actor.actor_id);

        new_actor.facing = orientation;
        let new_dest = WorldCell {
            actor: Some(new_actor),
            building: dest.unwrap().building.clone(),
            items: dest.unwrap().items.clone(),
        };

        let new_src = WorldCell {
            actor: None,
            building: src.building.clone(),
            items: src.items.clone(),
        };

        actor_ref.location = location + offsets[1] * orientation;
        actor_ref.orientation = orientation;
        game.world.setslice(
            location,
            orientation,
            &offsets,
            vec![Some(new_src), Some(new_dest)],
        );
    }
}

fn execute_take(
    location: Coordinate,
    orientation: AbsoluteDirection,
    game: &mut Game,
) {
    let offsets = vec![Coordinate { x: 0, y: 0 }];
    let cells = game.world.getslice(location, orientation, &offsets);

    let src = cells[0].unwrap();
    if src.items[0].is_none() {
        return;
    }

    let mut new_actor = src.actor.clone().unwrap();
    new_actor.facing = orientation;
    new_actor
        .inventory[0]  = Some(src.items[0].as_ref().unwrap().clone());

    let new_cell = WorldCell {
        actor: Some(new_actor),
        building: src.building.clone(),
        items: Default::default(),
    };

    game.world.setslice(location, orientation, &offsets, vec![Some(new_cell)]);
}


fn execute_use_cloner(
    idx: usize,
    location: Coordinate,
    orientation: AbsoluteDirection,
    game: &mut Game,
) {
    let offsets = vec![Coordinate { x: 0, y: 0 }, Coordinate { x: 0, y: 1 }];
    let cells = game.world.getslice(location, orientation, &offsets);

    let src = cells[0];
    let dst = cells[1];
    if src.is_none() || src.unwrap().actor.is_none() || src.is_none() {return;}
    let actor = src.unwrap().actor.as_ref().unwrap().clone();
    if actor.inventory[idx].is_none() {
        return;
    }
    let recorder = actor.inventory[idx].unwrap();
    if recorder.recording.is_none() {return;}
    let recording = recorder.recording.unwrap();

    if dst.is_none() || dst.unwrap().actor.is_some() {
        return;
    }

    let  new_actor_ref = ActorRef{
        location: location + offsets[1] * orientation,
        orientation: orientation,
        liveness: true,
        recording: recording,
        command_idx: 0,
    };
    let actor_id = game.actors.register_actor(new_actor_ref);
    let new_actor = Actor{
        facing: orientation,
        isplayer: false,
        actor_id: actor_id,
        inventory: Default::default()
    };

    let mut new_dest = dst.unwrap().clone();
    new_dest.actor = Some(new_actor);

    game.world.setslice(
        location,
        orientation,
        &offsets,
        vec![src.cloned(), Some(new_dest)],
    );

}