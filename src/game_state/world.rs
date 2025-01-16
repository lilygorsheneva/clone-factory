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

// Upstream HashMap with get_mut feature
use hashbrown::HashMap;

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

pub type World = WorldLayer<WorldCell>;
pub type WorldUpdate = WorldLayerUpdate<WorldCell>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let w = World::new(Coordinate { x: 1, y: 1 }, WorldCell::new());
        assert!(w.in_bounds(&Coordinate { x: 0, y: 0 }));
        assert!(w.get(&Coordinate { x: 0, y: 0 }).is_ok());
    }

    #[test]
    fn mutate() {
        let mut w = World::new(Coordinate { x: 1, y: 1 }, WorldCell::new());
        let location = Coordinate { x: 0, y: 0 };
        let oldcell = w.get(&location).unwrap();
        let newcell = WorldCell {
            actor: Some(Actor::new()),
            ..oldcell.clone()
        };
        assert_ne!(*oldcell, newcell); // Sanity check to ensure we actually mutate.
        w.mut_set(&location, &newcell).unwrap();
        assert_eq!(*w.get(&Coordinate { x: 0, y: 0 }).unwrap(), newcell);
    }

    #[test]
    fn update() {
        let mut w = World::new(Coordinate { x: 1, y: 1 }, WorldCell::new());
        let location = Coordinate { x: 0, y: 0 };
        let orientation = AbsoluteDirection::N;
        let offsets = [Coordinate { x: 0, y: 0 }];
        let mut update = WorldLayerUpdate::new();

        let mut cell = update.get(&w, &offsets[0]).unwrap().clone();

        assert!(cell.actor.is_none());
        cell.actor = Some(Actor::new());


        update.apply(&mut w).unwrap();
        assert!(w.get(&Coordinate { x: 0, y: 0 }).unwrap().actor.is_some());
    }
}
