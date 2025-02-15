//! Definitons for Actions performed by players or npcs.
use crate::actor::Actor;
use crate::buildings::{execute_use_building, Building};
use crate::datatypes::Coordinate;
use crate::direction::{AbsoluteDirection, Direction};
use crate::engine::tracking_worldlayer::TrackableId;
use crate::engine::update::{Delta, UpdatableContainer, UpdatableContainerDelta};
use crate::error::{
    Result,
    Status::{ActionFail, Error, OutOfBounds},
};
use crate::eventqueue::ActorEvent;
use crate::game_state::game::{Game, GameUpdate};
use crate::game_state::world::FloorTile;
use crate::inventory::Item;
use crate::static_data::RecipeDefiniton;
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
    Drop(usize),
    Use(usize),
    ActivateBuilding,
    Craft(&'static RecipeDefiniton),
    Wait,
}

pub fn execute_action(actor: TrackableId, action: Action, game: &Game) -> Result<GameUpdate> {
    let location = game.world.actors.get_location(&actor)?;
    let maybe_actor = game.world.actors.get(location)?;
    let actor = maybe_actor
        .as_ref()
        .ok_or(Error("No actor at expected coordinates"))?;

    let orientation = actor.facing.rotate(&action.direction);

    match action.action {
        SubAction::Move => execute_move(*location, orientation, game),
        SubAction::Take => execute_take(*location, orientation, game),
        SubAction::Use(idx) => execute_use_item(idx, *location, orientation, game),
        SubAction::Drop(idx) => execute_drop(idx, *location, orientation, game),
        SubAction::Craft(recipe) => execute_craft(recipe, *location, orientation, game), // _ => world,
        SubAction::ActivateBuilding => execute_use_building(*location, game),
        SubAction::Wait => Ok(GameUpdate::new()),
    }
}

fn execute_move(
    location: Coordinate,
    orientation: AbsoluteDirection,
    game: &Game,
) -> Result<GameUpdate> {
    let mut update: GameUpdate = GameUpdate::new();

    let [src_coord, dst_coord] = [Coordinate { x: 0, y: 0 }, Coordinate { x: 0, y: 1 }]
        .map(|i| Coordinate::as_offset(i, location, orientation));

    let src = update
        .world
        .actor_updates
        .get(&game.world.actors, &src_coord)?
        .ok_or(Error("Actor missing"))?;
    let dst = match update
        .world
        .actor_updates
        .get(&game.world.actors, &dst_coord)
    {
        Err(OutOfBounds) => Err(ActionFail("destination out of bounds")),
        Err(e) => Err(e),
        Ok(o) => Ok(o),
    }?;

    let dstfloor = update
        .world
        .floor_updates
        .get(&game.world.floor, &dst_coord)?;

    if dst.is_some() {
        return Err(ActionFail("destination occupied"));
    }

    match dstfloor {
        FloorTile::Water => {
            return Err(ActionFail("Destination impassable"));
        }
        _ => {}
    }

    let mut actor = src.clone();
    update.world.actor_updates.set(&src_coord, &None)?;

    actor.facing = orientation;
    update.world.actor_updates.set(&dst_coord, &Some(actor))?;

    Ok(update)
}

fn execute_take(
    location: Coordinate,
    orientation: AbsoluteDirection,
    game: &Game,
) -> Result<GameUpdate> {
    let mut update: GameUpdate = GameUpdate::new();

    let actor_cell = update
        .world
        .actor_updates
        .get(&game.world.actors, &location)?;
    let floor_cell = update
        .world
        .item_updates
        .get(&game.world.items, &location)?;

    match (actor_cell, floor_cell) {
        (None, _) => Err(Error("actor Missing")),
        (Some(_), [None]) => Err(ActionFail("no item to take")),
        (Some(actor), [Some(item)]) => {
            let mut actor = actor.clone();
            actor.facing = orientation;
            actor.inventory.insert(*item)?;
            update.world.actor_updates.set(&location, &Some(actor))?;
            update.world.item_updates.set(&location, &[None])?;
            Ok(update)
        }
    }
}

fn execute_drop(
    idx: usize,
    location: Coordinate,
    orientation: AbsoluteDirection,
    game: &Game,
) -> Result<GameUpdate> {
    let mut update: GameUpdate = GameUpdate::new();

    let actor_cell = update
        .world
        .actor_updates
        .get(&game.world.actors, &location)?;
    let floor_cell = update
        .world
        .item_updates
        .get(&game.world.items, &location)?;

    match (actor_cell, floor_cell) {
        (None, _) => Err(Error("actor Missing")),
        (Some(_), [Some(_)]) => Err(ActionFail("destination full")),
        (Some(actor), [None]) => {
            let mut actor = actor.clone();
            actor.facing = orientation;
            let item = actor
                .inventory
                .remove_idx(idx)
                .ok_or(ActionFail("No item in slot"))?;
            update.world.actor_updates.set(&location, &Some(actor))?;
            update.world.item_updates.set(&location, &[Some(item)])?;
            Ok(update)
        }
    }
}

fn execute_use_item(
    idx: usize,
    location: Coordinate,
    orientation: AbsoluteDirection,
    game: &Game,
) -> Result<GameUpdate> {
    let cell = game.world.actors.get(&location)?;
    let actor = cell.as_ref().ok_or(Error("Actor missing"))?;
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
    let mut update: GameUpdate = GameUpdate::new();

    let [src_coord, dst_coord] = [Coordinate { x: 0, y: 0 }, Coordinate { x: 0, y: 1 }]
        .map(|i| Coordinate::as_offset(i, location, orientation));

    let src = update
        .world
        .actor_updates
        .get(&game.world.actors, &src_coord);
    let dst = update
        .world
        .actor_updates
        .get(&game.world.actors, &dst_coord);

    match (src, dst) {
        (Err(err), _) => Err(err),
        (Ok(_), Err(OutOfBounds)) => Err(ActionFail("destination out of bounds")),
        (_, Err(err)) => Err(err),
        (Ok(None), _) => Err(Error("actor Missing")),
        (Ok(Some(_)), Ok(Some(_))) => Err(ActionFail("destination occupied")),
        (Ok(Some(source_actor)), Ok(None)) => {
            let mut source_actor = source_actor.clone();
            source_actor.facing = orientation;
            let recorder = source_actor.inventory.get_items()[idx].ok_or(ActionFail("no item"))?;
            let recordingid = recorder
                .recording
                .ok_or(Error("called use cloner on a non-recorder item"))?;

            let actor_id = update.world.actor_updates.get_next_id(&game.world.actors);

            let descriptor = game.data.actors.get("clone").unwrap();

            let mut new_actor = Actor::from_recording(descriptor, actor_id, game.recordings.get(recordingid));
            new_actor.facing = orientation;

            update
                .world
                .actor_updates
                .set(&src_coord, &Some(source_actor))?;
            update
                .world
                .actor_updates
                .set(&dst_coord, &Some(new_actor))?;
            update.eventqueue.this_turn.push_front(ActorEvent {
                actor: actor_id,
                recording: recordingid,
                recording_idx: 0,
            });
            Ok(update)
        }
    }
}

fn execute_construct(
    idx: usize,
    location: Coordinate,
    orientation: AbsoluteDirection,
    game: &Game,
) -> Result<GameUpdate> {
    let mut update: GameUpdate = GameUpdate::new();

    let actor_cell = update
        .world
        .actor_updates
        .get(&game.world.actors, &location)?;
    let floor_cell = update
        .world
        .building_updates
        .get(&game.world.buildings, &location)?
        .as_ref();

    match (actor_cell, floor_cell) {
        (None, _) => Err(Error("actor Missing")),
        (Some(_), Some(_)) => Err(ActionFail("destination full")),
        (Some(actor), None) => {
            let mut actor = actor.clone();
            actor.facing = orientation;
            let item = actor
                .inventory
                .remove_idx(idx)
                .ok_or(ActionFail("No item in slot"))?;
            let building_def = game
                .data
                .buildings
                .get(&item.definition.name)
                .ok_or(Error("called execute_construct on a non-building item"))?;
            update.world.actor_updates.set(&location, &Some(actor))?;
            update.world.building_updates.set(
                &location,
                &Some(Building {
                    definition: building_def,
                }),
            )?;
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
    let mut update: GameUpdate = GameUpdate::new();

    let actor_cell = update
        .world
        .actor_updates
        .get(&game.world.actors, &location)?;

    let product_definiton = game
        .data
        .items
        .get(&recipe.product)
        .ok_or(Error("product undefined"))?;
    let product: Item = Item::new(product_definiton, recipe.product_count as u16);

    if let Some(actor) = actor_cell {
        let mut actor = actor.clone();
        actor.facing = orientation;

        let inventory = &mut actor.inventory;

        for idx in 0..recipe.ingredients.len() {
            let ingredient_definiton = game
                .data
                .items
                .get(&recipe.ingredients[idx])
                .ok_or(Error("ingredient undefined"))?;

            let ingedient: Item = Item::new(ingredient_definiton, 1);
            for _ in 0..recipe.ingredient_counts[idx] as u16 {
                inventory.remove(ingedient)?;
            }
        }

        inventory.insert(product)?;

        update.world.actor_updates.set(&location, &Some(actor))?;
        Ok(update)
    } else {
        Err(Error("Actor Missing"))
    }
}

pub fn get_use_fn_table() -> HashMap<String, ItemUseFn> {
    let mut map: HashMap<String, ItemUseFn> = HashMap::new();

    map.insert("action_use_cloner".to_string(), execute_use_cloner);
    map.insert("action_construct".to_string(), execute_construct);

    map
}

#[cfg(test)]
mod tests {
    use crate::devtools;
    use crate::direction::Direction::Absolute;
    use crate::recording::Recording;
    use crate::static_data::Data;

    use super::*;

    #[test]
    fn move_action() {
        let data = Data::get_test_config();
        let mut game = Game::new(Coordinate { x: 1, y: 2 }, &data);

        let location = Coordinate { x: 0, y: 1 };
        assert!(game.spawn(&location).is_ok());
        let update = execute_move(location, AbsoluteDirection::S, &game);
        assert!(update.is_ok());
        update.unwrap().apply(&mut game).unwrap();

        let start = game.world.actors.get(&location).unwrap();
        let end = game.world.actors.get(&Coordinate { x: 0, y: 0 }).unwrap();

        assert!(start.is_none());
        assert!(end.is_some());
    }

    #[test]
    fn take_action() {
        let data = Data::get_test_config();
        let mut game = Game::new(Coordinate { x: 1, y: 1 }, &data);

        let location = Coordinate { x: 0, y: 0 };
        let item_def = data.items.get(&"raw_crystal".to_string()).unwrap();

        let foo = Item::new(item_def, 1);
        game.world.items.mut_set(&location, &[Some(foo)]).unwrap();

        assert!(game.spawn(&location).is_ok());

        let update = execute_take(location, AbsoluteDirection::S, &game);
        update.unwrap().apply(&mut game).unwrap();

        let actor = game.world.actors.get(&location).unwrap();
        assert_eq!(actor.unwrap().inventory.get_items()[0].unwrap(), foo);
        let floor = game.world.items.get(&location).unwrap();
        assert!(floor[0].is_none());
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

        let sample_recording_id = game.recordings.recordings.register_recording(Recording {
            command_list: actions,
            inventory: Default::default(),
            should_loop: true,
        });
        let cloner_def = data.items.get(&"basic_cloner".to_string()).unwrap();
        let new_cloner = Item::new_cloner(cloner_def, sample_recording_id);
        let update = devtools::grant_item(new_cloner, location, &game).unwrap();
        update.apply(&mut game).unwrap();

        let update = execute_use_cloner(0, location, AbsoluteDirection::N, &game);
        update.unwrap().apply(&mut game).unwrap();

        let start = game.world.actors.get(&location).unwrap();
        let end = game.world.actors.get(&Coordinate { x: 0, y: 1 }).unwrap();
        assert!(start.is_some());
        assert!(end.is_some());
    }

    #[test]
    fn craft() {
        let data = Data::get_test_config();
        let mut game = Game::new(Coordinate { x: 1, y: 2 }, &data);

        let location = Coordinate { x: 0, y: 0 };
        assert!(game.spawn(&location).is_ok());
        let item_def = data.items.get(&"raw_crystal".to_string()).unwrap();
        let update = devtools::grant_item(Item::new(item_def, 2), location, &game).unwrap();

        update.apply(&mut game).unwrap();

        let recipe = game.data.recipes.get(&"echo_crystal".to_string()).unwrap();
        let update = execute_craft(recipe, location, AbsoluteDirection::N, &game);
        update.unwrap().apply(&mut game).unwrap();

        let actor = game.world.actors.get(&location).unwrap();
        let crafted_item = actor.unwrap().inventory.get_items()[1].unwrap();

        assert_eq!(crafted_item.definition.text.name, "Echo Crystal");
    }
}
