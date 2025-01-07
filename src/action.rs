//! Definitons for Actions performed by players or npcs.
use crate::actor::{Actor, ActorRef};
use crate::datatypes::Coordinate;
use crate::direction::{AbsoluteDirection, Direction};
use crate::error::{
    Result,
    Status::{ActionFail, Error, NotFoundError},
};
use crate::game_state::game::{Game, GameUpdate};
use crate::game_state::world::WorldCell;
use crate::inventory::Item;
use crate::static_data::{Data, ItemDefiniton, RecipeDefiniton};
use std::collections::HashMap;

pub type ItemUseFn = fn(usize, Coordinate, AbsoluteDirection, &Game) -> Result<GameUpdate>;

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

pub fn execute_action(actor_ref: ActorRef, action: Action, game: &Game) -> Result<GameUpdate> {
    let orientation = actor_ref.orientation.rotate(&action.direction);

    match action.action {
        SubAction::Move => execute_move(actor_ref.location, orientation, game),
        SubAction::Take => execute_take(actor_ref.location, orientation, game),
        SubAction::Use(i) => execute_use_item(i, actor_ref.location, orientation, game),
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
    let cells = game
        .world
        .readslice(&mut update.world, location, orientation, &offsets);

    match cells {
        [None, _] => Err(Error("action performed on empty space")),
        [Some(WorldCell { actor: None, .. }), _] => Err(Error("actor Missing")),
        [Some(_), None] => Err(ActionFail("destination out of bounds")),
        [Some(_), Some(WorldCell {
            actor: Some(..), ..
        })] => Err(ActionFail("destination occupied")),
        [Some(
            src @ &mut WorldCell {
                actor: Some(..), ..
            },
        ), Some(dest @ WorldCell { actor: None, .. })] => {
            let actor: &mut Actor = src.actor.as_mut().unwrap();
            let actor_ref: &mut ActorRef = game
                .actors
                .db
                .read_actor(&mut update.actors, &actor.actor_id)
                .unwrap();
            actor_ref.location = location + offsets[1] * orientation;
            actor_ref.orientation = orientation;
            actor.facing = orientation;

            dest.actor = Some(actor.clone());
            src.actor = None;

            Ok(update)
        }
    }
}

fn execute_take(
    location: Coordinate,
    orientation: AbsoluteDirection,
    game: &Game,
) -> Result<GameUpdate> {
    let mut update: GameUpdate = game.new_update();

    let offsets = [Coordinate { x: 0, y: 0 }];
    let cells = game
        .world
        .readslice(&mut update.world, location, orientation, &offsets);

    match cells {
        [None] => Err(Error("action performed on empty space")),
        [Some(WorldCell { actor: None, .. })] => Err(Error("actor Missing")),
        [Some(
            src @ &mut WorldCell {
                actor: Some(..), ..
            },
        )] => {
            if src.items[0].is_none() {
                return Err(ActionFail("no item to take"));
            }

            let actor = src.actor.as_mut().unwrap();
            actor.facing = orientation;
            if let Some(item) = src.items[0] {
                actor.inventory.insert(item)?;
                src.items[0] = None;
                Ok(update)
            } else {
                Err(ActionFail("no  item to take"))
            }
        }
    }
}

fn execute_use_item(
    idx: usize,
    location: Coordinate,
    orientation: AbsoluteDirection,
    game: &Game,
) -> Result<GameUpdate> {
    let cell = game.world.get(&location).ok_or(Error("No worldcell"))?;
    let actor = cell.actor.ok_or(Error("Actor missing"))?;
    let item = actor.inventory.get_items()[idx].ok_or(ActionFail("no item"))?;
    let definition = game
        .data
        .items_by_id
        .get(&item.id)
        .ok_or(Error("item definiton not found"))?;
    let function = definition
        .on_use_fn
        .as_ref()
        .ok_or(ActionFail("Not a usable item"))?;
    function(idx, location, orientation, game)
}

fn execute_use_cloner(
    idx: usize,
    location: Coordinate,
    orientation: AbsoluteDirection,
    game: &Game,
) -> Result<GameUpdate> {
    let mut update: GameUpdate = game.new_update();

    let offsets = [Coordinate { x: 0, y: 0 }, Coordinate { x: 0, y: 1 }];
    let cells = game
        .world
        .readslice(&mut update.world, location, orientation, &offsets);

    match cells {
        [None, _] => Err(Error("action performed on empty space")),
        [Some(WorldCell { actor: None, .. }), _] => Err(Error("actor Missing")),
        [Some(_), None] => Err(ActionFail("destination out of bounds")),
        [Some(_), Some(WorldCell {
            actor: Some(..), ..
        })] => Err(ActionFail("destination occupied")),
        [Some(&mut WorldCell {
            actor: Some(actor), ..
        }), Some(dest @ WorldCell { actor: None, .. })] => match actor.inventory.get_items()[idx] {
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
                let actor_id = game
                    .actors
                    .db
                    .register_actor(&mut update.actors, new_actor_ref);
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
    game: &Game,
) -> Result<GameUpdate> {
    let mut update: GameUpdate = game.new_update();
    let offsets = [Coordinate { x: 0, y: 0 }];
    let mut cells = game
        .world
        .readslice(&mut update.world, location, orientation, &offsets);

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
            actor.inventory.insert(item)?;

            Ok(update)
        }
    }
}

fn execute_craft(
    recipe: &RecipeDefiniton,
    location: Coordinate,
    orientation: AbsoluteDirection,
    game: &Game,
) -> Result<GameUpdate> {
    let mut update: GameUpdate = game.new_update();
    let offsets = [Coordinate { x: 0, y: 0 }];

    let mut cells = game
        .world
        .readslice(&mut update.world, location, orientation, &offsets);

    let cell = cells[0].as_mut().ok_or(Error("out of bounds"))?;
    let actor = &mut cell.actor.as_mut().ok_or(Error("Actor missing"))?;
    let inventory = &mut actor.inventory;

    let product_definiton = game
        .data
        .items
        .get(&recipe.product)
        .ok_or(Error("product undefined"))?;
    let product: Item = Item::new(product_definiton.id, recipe.product_count as u16);

    for idx in 0..recipe.ingredients.len() {
        let ingredient_definiton = game
            .data
            .items
            .get(&recipe.ingredients[idx])
            .ok_or(Error("ingredient undefined"))?;
        let ingedient: Item = Item::new(
            ingredient_definiton.id,
            recipe.ingredient_counts[idx] as u16,
        );
        inventory.remove(ingedient)?;
    }

    inventory.insert(product)?;

    Ok(update)
}

pub fn get_use_fn_table() -> HashMap<String, Box<ItemUseFn>> {
    let mut map: HashMap<String, Box<ItemUseFn>> = HashMap::new();

    map.insert(
        "action_use_cloner".to_string(),
        Box::new(execute_use_cloner),
    );

    map
}

#[cfg(test)]
mod tests {
    use crate::datatypes::Recording;
    use crate::direction::Direction::Absolute;

    use super::*;

    #[test]
    fn move_action() {
        let data = Data::get_test_config();
        let mut game = Game::new(Coordinate { x: 1, y: 2 }, &data);

        let location = Coordinate { x: 0, y: 1 };
        assert!(game.spawn(&location).is_ok());
        let update = execute_move(location, AbsoluteDirection::S, &game);
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
        let data = Data::get_test_config();
        let mut game = Game::new(Coordinate { x: 1, y: 1 }, &data);

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

        let update = execute_take(location, AbsoluteDirection::S, &game);
        assert!(game.apply_update(update.unwrap()).is_ok());

        let cell = game.world.get(&location).unwrap();
        assert_eq!(
            cell.actor.as_ref().unwrap().inventory.get_items()[0].unwrap(),
            foo
        );
        assert!(cell.items[0].is_none());
    }

    #[test]
    fn use_cloner() {
        let data = Data::get_test_config();
        let mut game = Game::new(Coordinate { x: 1, y: 2 }, &data);

        let location = Coordinate { x: 0, y: 0 };
        assert!(game.spawn(&location).is_ok());

        let actions = vec![
            Action {
                direction: Absolute(AbsoluteDirection::N),
                action: SubAction::Move,
            },
            Action {
                direction: Absolute(AbsoluteDirection::N),
                action: SubAction::Move,
            },
        ];

        let sample_recording_id = game.recordings.register_recording(&Recording {
            command_list: actions,
            inventory: Default::default(),
        });
        let new_cloner = Item::new_cloner(4, sample_recording_id);
        let update = execute_action(
            game.actors.get_player().unwrap(),
            Action {
                direction: Absolute(AbsoluteDirection::N),
                action: SubAction::GrantItem(new_cloner),
            },
            &game,
        )
        .unwrap();
        game.apply_update(update).unwrap();

        let update = execute_use_cloner(0, location, AbsoluteDirection::N, &game);
        assert!(game.apply_update(update.unwrap()).is_ok());

        let start = game.world.get(&location);
        let end = game.world.get(&Coordinate { x: 0, y: 1 });
        assert!(start.is_some());
        assert!(start.unwrap().actor.is_some());

        assert!(end.is_some());
        assert!(end.unwrap().actor.is_some());
    }

    #[test]
    fn craft() {
        let data = Data::get_test_config();
        let mut game = Game::new(Coordinate { x: 1, y: 2 }, &data);

        let location = Coordinate { x: 0, y: 0 };
        assert!(game.spawn(&location).is_ok());
        let update = execute_action(
            game.actors.get_player().unwrap(),
            Action {
                direction: Absolute(AbsoluteDirection::N),
                action: SubAction::GrantItem(Item::new(0, 2)),
            },
            &game,
        )
        .unwrap();
        game.apply_update(update).unwrap();

        let recipe = game.data.recipes.get("echo_crystal").unwrap();
        let update = execute_craft(recipe, location, AbsoluteDirection::N, &game);
        assert!(game.apply_update(update.unwrap()).is_ok());

        let cell = game.world.get(&location).unwrap();
        let crafted_item = cell.actor.as_ref().unwrap().inventory.get_items()[1].unwrap();

        let target_item = Item::new(1, 1);

        assert_eq!(crafted_item, target_item);
    }
}
