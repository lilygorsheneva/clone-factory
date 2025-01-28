//! Datastructures to represent spatial data (e.g. a map of the world).

use crate::engine::tracking_worldlayer::{TrackableWorldLayer, TrackableWorldLayerDelta};
use crate::engine::update::{Updatable, Delta, UpdatableContainer};
use crate::engine::worldlayer::{WorldLayer, WorldLayerDelta};
use crate::error::Result;
use crate::inventory::Item;
use crate::paradox::Paradox;
use crate::{
    actor::Actor,
    datatypes::Coordinate,
};
use crate::buildings::Building;

pub type FloorInventory = [Option<Item>; 1];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FloorTile {
    Dirt,
    Water,
    Stone
}

#[derive(PartialEq, Debug, Clone)]
pub struct WorldCell<'a> {
    pub actor: Option<&'a Actor>,
    pub building: Option<&'a Building>,
    pub items: &'a [Option<Item>; 1],
    pub paradox: &'a Paradox,
    pub floor: &'a FloorTile
}

pub struct World {
    dimensions: Coordinate,
    pub actors: TrackableWorldLayer<Option<Actor>>,
    pub buildings: WorldLayer<Option<Building>>,
    pub items: WorldLayer<FloorInventory>,
    pub paradox: WorldLayer<Paradox>,
    pub floor: WorldLayer<FloorTile>
}

impl World {
    pub fn new(dimensions: Coordinate) -> World {
        World {
            dimensions: dimensions,
            actors: TrackableWorldLayer::new(dimensions, None),
            buildings: WorldLayer::new(dimensions, None),
            items: WorldLayer::new(dimensions, [None]),
            paradox: WorldLayer::new(dimensions, Paradox(0.0)),
            floor: WorldLayer::new(dimensions, FloorTile::Dirt),
        }
    }

    pub fn get_cell(&self, location: &Coordinate) -> Result<WorldCell> {
        let actor = self.actors.get(location)?;
        let building = self.buildings.get(location)?;
        let items = self.items.get(location)?;
        let paradox = self.paradox.get(location)?;
        let floor = self.floor.get(location)?;

        Ok(WorldCell{
            actor: actor.as_ref(),
            building: building.as_ref(),
            items,
            paradox,
            floor
        })
    }

    pub fn dimensions(&self) -> Coordinate {
        self.dimensions
    }
}

impl Updatable for World{}

#[derive(Debug)]
pub struct WorldUpdate {
    pub actor_updates: TrackableWorldLayerDelta<Option<Actor>>,
    pub building_updates: WorldLayerDelta<Option<Building>>,
    pub item_updates: WorldLayerDelta<FloorInventory>,
    pub paradox_updates: WorldLayerDelta<Paradox>,
    pub floor_updates: WorldLayerDelta<FloorTile>
}

impl Delta for WorldUpdate {
    type Target = World;
    fn new() -> WorldUpdate {
        WorldUpdate {
            actor_updates: TrackableWorldLayerDelta::new(),
            building_updates: WorldLayerDelta::new(),
            item_updates: WorldLayerDelta::new(),
            paradox_updates: WorldLayerDelta::new(),
            floor_updates: WorldLayerDelta::new()
        }
    }

    fn apply(&self, target: &mut World) -> Result<()> {
        self.actor_updates.apply(&mut target.actors)?;
        self.building_updates.apply(&mut target.buildings)?;
        self.item_updates.apply(&mut target.items)?;
        self.paradox_updates.apply(&mut target.paradox)?;
        self.floor_updates.apply(&mut target.floor)?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use crate::{engine::update::UpdatableContainerDelta, static_data::Data};

    use super::*;

    #[test]
    fn create() {
        let w = World::new(Coordinate { x: 1, y: 1 });
        assert!(w.actors.in_bounds(&Coordinate { x: 0, y: 0 }));
        assert!(w.actors.get(&Coordinate { x: 0, y: 0 }).is_ok());
    }

    #[test]
    fn mutate() {
        let data = Data::get_test_config();
        let mut w = World::new(Coordinate { x: 1, y: 1 });
        let location = Coordinate { x: 0, y: 0 };
        let oldcell = w.actors.get(&location).unwrap();
        let newcell = Some(Actor::new(data.actors.get("player").unwrap()));

        assert_ne!(*oldcell, newcell); // Sanity check to ensure we actually mutate.
        w.actors.mut_set(&location, &newcell).unwrap();
        assert_eq!(*w.actors.get(&Coordinate { x: 0, y: 0 }).unwrap(), newcell);
    }

    #[test]
    fn update() {
        let data = Data::get_test_config();

        let mut w = World::new(Coordinate { x: 1, y: 1 });
        let location = Coordinate { x: 0, y: 0 };
        let mut update = WorldUpdate::new();

        let mut actor = update
            .actor_updates
            .get(&w.actors, &location)
            .unwrap()
            .clone();

        assert!(actor.is_none());
        let newcell = Some(Actor::new(data.actors.get("player").unwrap()));
        update.actor_updates.set(&location, &actor).unwrap();

        update.apply(&mut w).unwrap();
        assert!(w.actors.get(&Coordinate { x: 0, y: 0 }).unwrap().is_some());
    }
}
