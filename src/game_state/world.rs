//! Datastructures to represent spatial data (e.g. a map of the world).

use crate::engine::update::{Updatable, Update};
use crate::engine::worldlayer::{WorldLayer, WorldLayerUpdate};
use crate::error::{Result, Status::StateUpdateError};
use crate::inventory::Item;
use crate::{
    actor::Actor,
    datatypes::{Building, Coordinate},
    direction::AbsoluteDirection,
};

pub type FloorInventory = [Option<Item>; 1];

#[derive(PartialEq, Debug, Clone)]
pub struct WorldCell {
    pub actor: Option<Actor>,
    pub building: Option<Building>,
    pub items: [Option<Item>; 1],
}

impl WorldCell {
    pub fn new() -> WorldCell {
        WorldCell {
            actor: None,
            building: None,
            items: Default::default(),
        }
    }
}

pub struct World {
    dimensions: Coordinate,
    pub actors: WorldLayer<Option<Actor>>,
    pub buildings: WorldLayer<Option<Building>>,
    pub items: WorldLayer<FloorInventory>,
}

impl World {
    pub fn new(dimensions: Coordinate) -> World {
        World {
            dimensions: dimensions,
            actors: WorldLayer::new(dimensions, None),
            buildings: WorldLayer::new(dimensions, None),
            items: WorldLayer::new(dimensions, [None]),
        }
    }
}

#[derive(Debug)]
pub struct WorldUpdate {
    pub actor_updates: WorldLayerUpdate<Option<Actor>>,
    pub building_updates: WorldLayerUpdate<Option<Building>>,
    pub item_updates: WorldLayerUpdate<FloorInventory>,
}

impl WorldUpdate {
    pub fn new() -> WorldUpdate {
        WorldUpdate {
            actor_updates: WorldLayerUpdate::new(),
            building_updates: WorldLayerUpdate::new(),
            item_updates: WorldLayerUpdate::new(),
        }
    }

    pub fn apply(&self, target: &mut World) -> Result<()> {
        self.actor_updates.apply(&mut target.actors)?;
        self.building_updates.apply(&mut target.buildings)?;
        self.item_updates.apply(&mut target.items)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let w = World::new(Coordinate { x: 1, y: 1 });
        assert!(w.actors.in_bounds(&Coordinate { x: 0, y: 0 }));
        assert!(w.actors.get(&Coordinate { x: 0, y: 0 }).is_ok());
    }

    #[test]
    fn mutate() {
        let mut w = World::new(Coordinate { x: 1, y: 1 });
        let location = Coordinate { x: 0, y: 0 };
        let oldcell = w.actors.get(&location).unwrap();
        let newcell = Some(Actor::new());

        assert_ne!(*oldcell, newcell); // Sanity check to ensure we actually mutate.
        w.actors.mut_set(&location, &newcell).unwrap();
        assert_eq!(*w.actors.get(&Coordinate { x: 0, y: 0 }).unwrap(), newcell);
    }

    #[test]
    fn update() {
        let mut w = World::new(Coordinate { x: 1, y: 1 });
        let location = Coordinate { x: 0, y: 0 };
        let orientation = AbsoluteDirection::N;
        let offsets = [Coordinate { x: 0, y: 0 }];
        let mut update = WorldUpdate::new();

        let mut actor = update
            .actor_updates
            .get(&w.actors, &offsets[0])
            .unwrap()
            .clone();

        assert!(actor.is_none());
        actor = Some(Actor::new());
        update.actor_updates.set(&offsets[0], actor);

        update.apply(&mut w).unwrap();
        assert!(w.actors.get(&Coordinate { x: 0, y: 0 }).unwrap().is_some());
    }
}
