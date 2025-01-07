use crate::action::{Action, SubAction};
use crate::datatypes::{Coordinate, Recording};
use crate::direction::RelativeDirection;
use crate::game_state::game::{Game, GameUpdate};
use crate::game_state::world::WorldCell;
use crate::inventory::Item;
use crate::error::{Result,Status::Error};

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
    let offsets = [Coordinate { x: 0, y: 0 }];
    let mut cells = game
        .world
        .readslice(&mut update.world, location, crate::direction::AbsoluteDirection::N, &offsets);

    match &mut cells[0] {
        None => Err(Error("action performed on empty space")),
        Some(WorldCell { actor: None, .. }) => Err(Error("actor Missing")),
        Some(
            src @ WorldCell {
                actor: Some(..), ..
            },
        ) => {
            let actor = src.actor.as_mut().unwrap();

            actor.inventory.insert(item)?;

            Ok(update)
        }
    }
}