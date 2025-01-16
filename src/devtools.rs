use crate::action::{Action, SubAction};
use crate::datatypes::{Coordinate, Recording};
use crate::direction::RelativeDirection;
use crate::engine::update::Update;
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
        inventory: Default::default()
    }
}


pub fn grant_item(
    item: Item,
    location: Coordinate,
    game: &Game,
) -> Result<GameUpdate> {
    let mut update: GameUpdate = game.new_update();
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