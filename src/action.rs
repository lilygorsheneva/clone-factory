//! Definitons for Actions performed by players or npcs.
use crate::actor::{Actor, ActorRef};
use crate::static_data::{StaticData, ItemDefiniton, RecipeDefiniton};
use crate::datatypes::Coordinate;
use crate::direction::{AbsoluteDirection, Direction};
use crate::error::{
    Result,
    Status::{ActionFail, Error},
};
use crate::game_state::game::{Game, GameUpdate};
use crate::inventory::Item;
use crate::game_state::world::WorldCell;
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
}

pub fn execute_action(actor_ref: ActorRef, action: Action, game: &Game) -> Result<GameUpdate> {
    let orientation = actor_ref.orientation.rotate(&action.direction);

    match action.action {
        SubAction::Move => execute_move(actor_ref.location, orientation, game),
        SubAction::Take => execute_take(actor_ref.location, orientation, game),
        SubAction::Use(i) => execute_use_item(i, actor_ref.location, orientation, game),
        // SubAction::Record => execute_recording(actor_ref.location, orientation, game),
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
    let definition = item.definition;
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

    let product_definiton = game.data.items.get(&recipe.product).ok_or(Error("product undefined"))?;
    let product: Item = Item::new(product_definiton, recipe.product_count as u16);

    for idx in 0..recipe.ingredients.len() {
        let ingredient_definiton = game.data.items.get(&recipe.ingredients[idx]).ok_or(Error("ingredient undefined"))?;
        let ingedient : Item = Item::new(ingredient_definiton, recipe.ingredient_counts[idx] as u16);
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
    use crate::devtools;
    use crate::direction::Direction::Absolute;

    use super::*;

    #[test]
    fn move_action() {
        let data = StaticData::get_test_config();
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
        let data = StaticData::get_test_config();
        let mut game = Game::new(Coordinate { x: 1, y: 1 }, &data);

        let location = Coordinate { x: 0, y: 0 };
        let item_def = data.items.get(&"raw_crystal".to_string()).unwrap();

        let foo = Item::new(item_def, 1);
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
        let data = StaticData::get_test_config();
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
        let cloner_def = data.items.get(&"basic_cloner".to_string()).unwrap();
        let new_cloner = Item::new_cloner(cloner_def, sample_recording_id);
        let update =devtools::grant_item(new_cloner, location, &game).unwrap();
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
        let data = StaticData::get_test_config();
        let mut game = Game::new(Coordinate { x: 1, y: 2 }, &data);

        let location = Coordinate { x: 0, y: 0 };
        assert!(game.spawn(&location).is_ok());
        let item_def = data.items.get(&"raw_crystal".to_string()).unwrap();
        let update =devtools::grant_item(Item::new(item_def, 2), location, &game).unwrap();

        game.apply_update(update).unwrap();
        

        let recipe = game.data.recipes.get(&"echo_crystal".to_string()).unwrap();
        let update = execute_craft(recipe, location, AbsoluteDirection::N, &game);
        assert!(game.apply_update(update.unwrap()).is_ok());

        let cell = game.world.get(&location).unwrap();
        let crafted_item = cell.actor.as_ref().unwrap().inventory.get_items()[1].unwrap();
        
        assert_eq!(crafted_item.definition.name, "Echo Crystal");
    }
}
