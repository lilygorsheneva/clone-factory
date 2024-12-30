use crate::datatypes::ActorRef;
use crate::datatypes::Coordinate;
use crate::direction::AbsoluteDirection;
use crate::direction::Direction;
use crate::world::World;
use crate::world::WorldCell;

pub struct Action {
    pub direction: Direction,
    pub action: SubAction,
}

pub enum SubAction {
    Move,
    // Take,
    // Drop,
    // Use(u8),
    // Craft(String),
    // Record,
    // EndRecording,
}

pub fn execute_action(actor_ref: &mut ActorRef, action: Action, world: World) -> World{
    let cell = world.get(&actor_ref.location).unwrap();
    let actor = cell.actor.as_ref().unwrap();
    let orientation = actor.facing.rotate(&action.direction);

    match action.action {
        SubAction::Move => execute_move(actor_ref, actor_ref.location, orientation, world),
        // _ => world,
    }
}

fn execute_move(
    actor_ref: &mut ActorRef,
    location: Coordinate,
    orientation: AbsoluteDirection,
    world: World,
) -> World {
    let offsets = vec![Coordinate { x: 0, y: 0 },Coordinate { x: 1, y: 0 }];
    let cells = world.getslice(location, orientation, &offsets);

    let src = cells[0].unwrap();
    let dest = cells[1];

    if dest.is_none() || dest.unwrap().actor.is_some() {
        // fail
        world
    } else {
        let mut new_actor = src.actor.clone().unwrap();
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

        actor_ref.location = actor_ref.location + offsets[1] * orientation;

        return world.setslice(
            location,
            orientation,
            &offsets,
            vec![Some(new_src), Some(new_dest)],
        );
    }
}
