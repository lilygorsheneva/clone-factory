use std::collections::HashMap;

use crate::paradox::Paradox;
use crate::engine::update::{Delta, UpdatableContainer, UpdatableContainerDelta};
use crate::game_state::game::Game;
use crate::inventory::Item;
use crate::static_data::ObjectDescriptor;
use crate::{datatypes::Coordinate, game_state::game::GameUpdate};
use crate::error::{Result, Status::{ActionFail,Error}};

pub type BuildingUseFn = fn(Coordinate, &Game) -> Result<GameUpdate>;


#[derive(PartialEq, Debug, Clone)]
pub struct Building {
    pub definition: &'static ObjectDescriptor,
}



pub fn execute_use_building(
    location: Coordinate,
    game: &Game,
) -> Result<GameUpdate> {
    let cell = game.world.buildings.get(&location)?;
    let building = cell.as_ref().ok_or(ActionFail("No building"))?;
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
    let item = floor[0].ok_or(ActionFail("no item to digitize"))?;
    let value = item.definition.score_value.unwrap_or(0);

            
    update.score.score += value;    
    update.world.item_updates.set(&location, &[None])?;


    let paradox = update.world.paradox_updates.get(&game.world.paradox, &location)?;
    let mut new_paradox =  paradox.0 - value as f64;
    if new_paradox < 0.0 {
        new_paradox = 0.0;
    }
    update.world.paradox_updates.set(&location, &Paradox(new_paradox))?;
    Ok(update)
        
}

pub fn get_building_fn_table() -> HashMap<String, BuildingUseFn> {
    let mut map: HashMap<String, BuildingUseFn> = HashMap::new();

    map.insert(
        "building_mine".to_string(),
        use_ore_deposit,
    );
    map.insert(
        "building_digitize".to_string(),
        use_matter_digitizer,
    );

    map
}
