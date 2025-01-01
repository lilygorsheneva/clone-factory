use crate::actor::{self, Actor, ActorRef};
use crate::datatypes::{Coordinate, Item};
use crate::direction::{AbsoluteDirection, Direction};
use crate::error::{
    Result,
    Status::{ActionFail, Error},
};
use crate::game::Game;
use crate::world::WorldCell;

#[derive(Clone, Copy)]
pub struct Action {
    pub direction: Direction,
    pub action: SubAction,
}

#[derive(Clone, Copy)]
pub enum SubAction {
    Move,
    Take,
    // Drop,
    Use(usize),
    // Craft(String),
    // Record,
    GrantItem(Item),
}

pub fn execute_action(actor_ref: ActorRef, action: Action, game: &mut Game) -> Result<()> {
    let orientation = actor_ref.orientation.rotate(&action.direction);

    match action.action {
        SubAction::Move => execute_move(actor_ref.location, orientation, game),
        SubAction::Take => execute_take(actor_ref.location, orientation, game),
        SubAction::Use(i) => execute_use_cloner(i, actor_ref.location, orientation, game),
        // SubAction::Record => execute_recording(actor_ref.location, orientation, game),
        SubAction::GrantItem(i) => execute_grant_item(i, actor_ref.location, orientation, game),
        // _ => world,
    }
}

fn execute_move(
    location: Coordinate,
    orientation: AbsoluteDirection,
    game: &mut Game,
) -> Result<()> {
    let offsets = vec![Coordinate { x: 0, y: 0 }, Coordinate { x: 0, y: 1 }];
    let cells = game.world.getslice(location, orientation, &offsets);

    match (cells[0], cells[1]) {
        (None, _) => Err(Error("action performed on empty space")),
        (Some(WorldCell { actor: None, .. }), _) => Err(Error("actor Missing")),
        (Some(_), None)
        | (
            Some(_),
            Some(WorldCell {
                actor: Some(..), ..
            }),
        ) => Err(ActionFail),
        (
            Some(
                src @ WorldCell {
                    actor: Some(actor), ..
                },
            ),
            Some(dest @ WorldCell { actor: None, .. }),
        ) => {
            let mut new_actor = actor.clone();
            let actor_ref: &mut ActorRef = game.actors.get_mut_actor(new_actor.actor_id);

            new_actor.facing = orientation;
            let new_dest = WorldCell {
                actor: Some(new_actor),
                building: dest.building.clone(),
                items: dest.items.clone(),
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
            )
        }
    }
}

fn execute_take(
    location: Coordinate,
    orientation: AbsoluteDirection,
    game: &mut Game,
) -> Result<()> {
    let offsets = vec![Coordinate { x: 0, y: 0 }];
    let cells = game.world.getslice(location, orientation, &offsets);

    match cells[0] {
        None => Err(Error("action performed on empty space")),
        Some(WorldCell { actor: None, .. }) => Err(Error("actor Missing")),
        Some(
            src @ WorldCell {
                actor: Some(actor), ..
            },
        ) => {
            if src.items[0].is_none() {
                return Err(ActionFail);
            }

            let mut new_actor = actor.clone();
            new_actor.facing = orientation;
            new_actor.inventory[0] = src.items[0].clone();

            let new_cell = WorldCell {
                actor: Some(new_actor),
                building: src.building.clone(),
                items: Default::default(),
            };

            game.world
                .setslice(location, orientation, &offsets, vec![Some(new_cell)])
        }
    }
}

fn execute_use_cloner(
    idx: usize,
    location: Coordinate,
    orientation: AbsoluteDirection,
    game: &mut Game,
) -> Result<()> {
    let offsets = vec![Coordinate { x: 0, y: 0 }, Coordinate { x: 0, y: 1 }];
    let cells = game.world.getslice(location, orientation, &offsets);

    match (cells[0], cells[1]) {
        (None, _) => Err(Error("action performed on empty space")),
        (Some(WorldCell { actor: None, .. }), _) => Err(Error("actor Missing")),
        (Some(_), None)
        | (
            Some(_),
            Some(WorldCell {
                actor: Some(..), ..
            }),
        ) => Err(ActionFail),
        (
            Some(
                src @ WorldCell {
                    actor: Some(actor), ..
                },
            ),
            Some(dest @ WorldCell { actor: None, .. }),
        ) => match actor.inventory[idx] {
            Some(Item {
                recording: Some(recordingid),
                ..
            }) => {
                let new_actor_ref = ActorRef {
                    location: location + offsets[1] * orientation,
                    orientation: orientation,
                    live: true,
                    isplayer: false,
                    recording: recordingid,
                    command_idx: 0,
                };
                let actor_id = game.actors.register_actor(new_actor_ref);
                let mut new_actor = Actor::from_recording(game.recordings.get(recordingid));
                new_actor.facing = orientation;
                new_actor.actor_id = actor_id;

                let mut new_dest = dest.clone();
                new_dest.actor = Some(new_actor);

                game.world.setslice(
                    location,
                    orientation,
                    &offsets,
                    vec![Some(src.clone()), Some(new_dest)],
                )
            }
            _ => Err(ActionFail),
        },
    }
}

fn execute_grant_item(
    item: Item,
    location: Coordinate,
    orientation: AbsoluteDirection,
    game: &mut Game,
) -> Result<()> {
    let offsets = vec![Coordinate { x: 0, y: 0 }];
    let cells = game.world.getslice(location, orientation, &offsets);

    match cells[0] {
        None => Err(Error("action performed on empty space")),
        Some(WorldCell { actor: None, .. }) => Err(Error("actor Missing")),
        Some(
            src @ WorldCell {
                actor: Some(actor), ..
            },
        ) => {
            let mut new_actor = actor.clone();
            new_actor.facing = orientation;
            new_actor.inventory[1] = Some(item);

            let new_cell = WorldCell {
                actor: Some(new_actor),
                building: src.building.clone(),
                items: Default::default(),
            };

            game.world
                .setslice(location, orientation, &offsets, vec![Some(new_cell)])
        }
    }
}
