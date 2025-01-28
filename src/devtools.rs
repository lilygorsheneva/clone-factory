use crate::action::{Action, SubAction};
use crate::datatypes::Coordinate;
use crate::engine::tracking_worldlayer::TrackableId;
use crate::recording::Recording;
use crate::direction::RelativeDirection;
use crate::engine::update::{Delta, UpdatableContainerDelta};
use crate::game_state::game::{Game, GameUpdate};
use crate::inventory::Item;
use crate::error::{Result,Status::Error};

#[allow(dead_code)]
pub fn make_sample_recording() -> Recording {
    Recording {
        command_list: vec![
            Action {
                direction: crate::direction::Direction::Relative(RelativeDirection::F),
                action: SubAction::Move,
            },
            Action {
                direction: crate::direction::Direction::Relative(RelativeDirection::R),
                action: SubAction::Move,
            },
            Action {
                direction: crate::direction::Direction::Relative(RelativeDirection::R),
                action: SubAction::Move,
            },
            Action {
                direction: crate::direction::Direction::Relative(RelativeDirection::R),
                action: SubAction::Move,
            },
            Action {
                direction: crate::direction::Direction::Relative(RelativeDirection::R),
                action: SubAction::Move,
            },
        ],
        inventory: Default::default(),
        should_loop: true
    }
}


pub fn grant_item(
    item: Item,
    location: Coordinate,
    game: &Game,
) -> Result<GameUpdate> {
    let mut update: GameUpdate = GameUpdate::new();
    let actor  = update.world.actor_updates.get(&game.world.actors, &location)?;

    match actor{
        None => Err(Error("actor Missing")),
        Some(actor 
        ) => {
            let mut actor = actor.clone();
            actor.inventory.insert(item)?;

            update.world.actor_updates.set(&location, &Some(actor))?;
            Ok(update)
        }
    }
}

pub fn remove_item(
    item: Item,
    location: Coordinate,
    game: &Game,
) -> Result<GameUpdate> {
    let mut update: GameUpdate = GameUpdate::new();
    let actor  = update.world.actor_updates.get(&game.world.actors, &location)?;

    match actor{
        None => Err(Error("actor Missing")),
        Some(actor 
        ) => {
            let mut actor = actor.clone();
            actor.inventory.remove(item)?;

            update.world.actor_updates.set(&location, &Some(actor))?;
            Ok(update)
        }
    }
}

pub fn despawn_actor(
    actorid: TrackableId,
    game: &Game
) ->  Result<GameUpdate> {
    let mut update: GameUpdate = GameUpdate::new();
    let location = game.world.actors.get_location(&actorid)?;
    let actor  = update.world.actor_updates.get(&game.world.actors, &location)?;
    if actor.is_none() {return Err(Error("actor missing"))}
    update.world.actor_updates.set(&location, &None)?;
    update.world.actor_updates.remove(actorid);
    Ok(update)
}
