use crate::actor::{Actor, ActorRef};
use crate::datatypes::{Coordinate, Item};
use crate::direction::{AbsoluteDirection, Direction};
use crate::error::{
    Result,
    Status::{ActionFail, Error},
};
use crate::game::{Game, GameUpdate};
use crate::world::WorldCell;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Action {
    pub direction: Direction,
    pub action: SubAction,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SubAction {
    Move,
    Take,
    // Drop,
    Use(usize),
    // Craft(String),
    // Record,
    GrantItem(Item),
}

pub fn execute_action(actor_ref: ActorRef, action: Action, game: &mut Game) -> Result<GameUpdate> {
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
    game: &Game,
) -> Result<GameUpdate> {
    let mut update: GameUpdate = game.new_update();


    let offsets = [Coordinate { x: 0, y: 0 }, Coordinate { x: 0, y: 1 }];
    let mut binding  = game.world.readslice(&mut update.world, location, orientation, &offsets);
    let cells = binding.split_at_mut(1);

    match (&mut cells.0[0], &mut cells.1[0]) {
        (None, _) => Err(Error("action performed on empty space")),
        (Some(WorldCell { actor: None, .. }), _) => Err(Error("actor Missing")),
        (Some(_), None)
        | (
            Some(_),
            Some(WorldCell {
                actor: Some(..), ..
            }),
        ) => Err(ActionFail("move into nonexistent cell")),
        (
            Some(
                 src @ &mut WorldCell {
                    actor: Some(..), ..
                },
            ),
            Some(dest @ WorldCell { actor: None, .. }),
        ) => {
            let actor = src.actor.as_mut().unwrap();
            let mut actor_ref: ActorRef = game.actors.get_actor(actor.actor_id);
            actor_ref.location = location + offsets[1] * orientation;
            actor_ref.orientation = orientation;
            actor.facing = orientation;

            game.actors
                .db
                .update_actor(&mut update.actors, actor.actor_id, actor_ref);

            dest.actor = Some(actor.clone());
            src.actor = None;
            
            Ok(update)
        }
    }
}

fn execute_take(
    location: Coordinate,
    orientation: AbsoluteDirection,
    game: &mut Game,
) -> Result<GameUpdate> {   
     let mut update: GameUpdate = game.new_update();

    let offsets = [Coordinate { x: 0, y: 0 }];
    let mut cells  = game.world.readslice(&mut update.world, location, orientation, &offsets);

    match &mut cells[0] {
        None => Err(Error("action performed on empty space")),
        Some(WorldCell { actor: None, .. }) => Err(Error("actor Missing")),
        Some(
            src @ &mut WorldCell {
                actor: Some(..), ..
            },
        ) => {
            if src.items[0].is_none() {
                return Err(ActionFail("no item to take"));
            }

            let actor = src.actor.as_mut().unwrap();
            actor.facing = orientation;
            actor.inventory[0] = src.items[0].clone();
            src.items[0] = None;

            Ok(update)
        }
    }
}

fn execute_use_cloner(
    idx: usize,
    location: Coordinate,
    orientation: AbsoluteDirection,
    game: &mut Game,
) -> Result<GameUpdate> {
    let mut update: GameUpdate = game.new_update();

    let offsets = [Coordinate { x: 0, y: 0 }, Coordinate { x: 0, y: 1 }];
    let mut binding  = game.world.readslice(&mut update.world, location, orientation, &offsets);
    let cells = binding.split_at_mut(1);

    match (&mut cells.0[0], &mut cells.1[0]) {
        (None, _) => Err(Error("action performed on empty space")),
        (Some(WorldCell { actor: None, .. }), _) => Err(Error("actor Missing")),
        (Some(_), None)
        | (
            Some(_),
            Some(WorldCell {
                actor: Some(..), ..
            }),
        ) => Err(ActionFail("cloning into nonexistent cell")),
        (
            Some(
                src @ &mut WorldCell {
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
                let actor_id = game.actors.mut_register_actor(new_actor_ref);
                let mut new_actor = Actor::from_recording(game.recordings.get(recordingid));
                new_actor.facing = orientation;
                new_actor.actor_id = actor_id;

                dest.actor = Some(new_actor);
                Ok(update)
            }
            _ => Err(ActionFail("no item to use")),
        },
    }
}

fn execute_grant_item(
    item: Item,
    location: Coordinate,
    orientation: AbsoluteDirection,
    game: &mut Game,
) -> Result<GameUpdate> {
    let mut update: GameUpdate = game.new_update();
    let offsets = [Coordinate { x: 0, y: 0 }];
    let mut cells  = game.world.readslice(&mut update.world, location, orientation, &offsets);

    match &mut cells[0] {
        None => Err(Error("action performed on empty space")),
        Some(WorldCell { actor: None, .. }) => Err(Error("actor Missing")),
        Some(
            src @ WorldCell {
                actor: Some(..), ..
            },
        ) => {
            let actor = src.actor.as_mut().unwrap(); 
            
            actor.facing = orientation;
            actor.inventory[1] = Some(item);

            Ok(update)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_action() {
        let mut game = Game::new(Coordinate { x: 1, y: 2 });

        let location = Coordinate { x: 0, y: 1 };
        assert!(game.spawn(&location).is_ok());
        let update = execute_move(location, AbsoluteDirection::S, &mut game);
        assert!(update.is_ok());
        assert!(game.apply_update(update.unwrap()).is_ok());

        let start = game.world.get(&location);
        let end = game.world.get(&Coordinate { x: 0, y: 0 });
        assert!(start.is_some());
        assert!(start.unwrap().actor.is_none());

        assert!(end.is_some());
        assert!(end.unwrap().actor.is_some());
    }

    #[test]
    fn take_action() {
        let mut game = Game::new(Coordinate { x: 1, y: 1 });

        let location = Coordinate { x: 0, y: 0 };
        let foo = Item::new(0, 1);
        game.world
            .mut_set(
                &location,
                Some(WorldCell {
                    actor: None,
                    building: None,
                    items: [Some(foo)],
                }),
            )
            .unwrap();

        assert!(game.spawn(&location).is_ok());

        let update = execute_take(location, AbsoluteDirection::S, &mut game);
        assert!(game.apply_update(update.unwrap()).is_ok());

        let cell = game.world.get(&location).unwrap();
        assert_eq!(cell.actor.as_ref().unwrap().inventory[0].unwrap(), foo);
        assert!(cell.items[0].is_none());
    }

    #[test]
    fn use_cloner() {
        let mut game = Game::new(Coordinate { x: 1, y: 2 });

        let location = Coordinate { x: 0, y: 0 };
        assert!(game.spawn(&location).is_ok());

        let update = execute_use_cloner(1, location, AbsoluteDirection::N, &mut game);
        assert!(game.apply_update(update.unwrap()).is_ok());

        let start = game.world.get(&location);
        let end = game.world.get(&Coordinate { x: 0, y: 1 });
        assert!(start.is_some());
        assert!(start.unwrap().actor.is_some());

        assert!(end.is_some());
        assert!(end.unwrap().actor.is_some());
    }
}
