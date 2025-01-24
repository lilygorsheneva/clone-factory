use ratatui::buffer;

use crate::actor;
use crate::direction::AbsoluteDirection;
use crate::engine::update::{self, Delta, UpdatableContainer, UpdatableContainerDelta};
use crate::game_state::game::Game;
use crate::inventory::Item;
use crate::{datatypes::Coordinate, game_state::game::GameUpdate, static_data::BuildingDefinition};
use crate::error::{Result, Status::{ActionFail,Error}};

pub type BuildingUseFn = fn(Coordinate, &Game) -> Result<GameUpdate>;


#[derive(PartialEq, Debug, Clone)]
pub struct Building {
    definition: &'static BuildingDefinition,
    facing: AbsoluteDirection,
}



fn execute_use_building(
    location: Coordinate,
    game: &Game,
) -> Result<GameUpdate> {
    let cell = game.world.buildings.get(&location)?;
    let building = cell.as_ref().ok_or(Error("Actor missing"))?;
    let definition = building.definition;
    let function = definition
        .on_interact_fn
        .as_ref()
        .ok_or(ActionFail("Not a usable item"))?;
    function(location, game)
}

fn use_ore_deposit(
    location: Coordinate,
    game: &Game,
) -> Result<GameUpdate> {
    let mut update = GameUpdate::new();

    let building = update.world.building_updates.get(&game.world.buildings, &location)?;
    let floor = update.world.item_updates.get(&game.world.items, &location)?;
    

    match (building, floor) {
        (_, [Some(_)]) => Err(ActionFail("destination occupied")),
        (_, [None]) => {
            let oredef = game.data.items.get(&"raw_crystal".to_string()).ok_or(Error("item definition not found"))?;
            let ore = Item::new(oredef, 1);
            update.world.item_updates.set(&location, &[Some(ore)])?;
            Ok(update)
        }
    }
}

fn use_matter_digitizer(
    location: Coordinate,
    game: &Game,
) -> Result<GameUpdate> {
    let mut update = GameUpdate::new();

    let floor = update.world.item_updates.get(&game.world.items, &location)?;
    

    match floor {
        [None] => Err(ActionFail("no item to digitize")),
        [Some(item)] => {
            update.score.0 += item.definition.score_value.unwrap_or(0);
            update.world.item_updates.set(&location, &[None])?;
            Ok(update)
        }
    }
}