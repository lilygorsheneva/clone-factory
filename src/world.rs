use crate::{
    actor::{Actor, ActorRef},
    datatypes::{Building, Coordinate, Item},
    direction::AbsoluteDirection,
};
use core::panic;
use std::{collections::VecDeque, rc::Rc};

#[derive(Clone)]
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
    pub dimensions: Coordinate,
    data: rpds::Vector<WorldCell>,
}

impl World {
    fn in_bounds(&self, location: &Coordinate) -> bool {
        location.x >= 0
            && location.x < self.dimensions.x
            && location.y >= 0
            && location.y < self.dimensions.y
    }

    fn coord_to_idx(&self, location: &Coordinate) -> usize {
        (location.x + location.y * self.dimensions.x) as usize
    }

    pub fn get(&self, location: &Coordinate) -> Option<&WorldCell> {
        // In_rect misbehaves for some reason.
        if self.in_bounds(&location) {
            return Some(&self.data[self.coord_to_idx(location)]);
        } else {
            return None;
        }
    }

    pub fn set(&mut self, location: &Coordinate, data: Option<WorldCell>) {
        match data {
            Some(cell) => {
                if self.in_bounds(&location) {
                    self.data = self
                        .data
                        .set((&self).coord_to_idx(&location), cell)
                        .unwrap();
                } else {
                    panic!("Attempting to set out of bounds cell")
                }
            }
            None => {
                if self.in_bounds(&location) {
                    panic!("Setting in-bounds cell to None")
                } else {
                    return ();
                }
            }
        }
    }

    pub fn new(dimensions: Coordinate) -> World {
        let mut datavec = rpds::Vector::new();
        for i in 0..(dimensions.x * dimensions.y) {
            datavec = datavec.push_back(WorldCell::new());
        }
        World {
            dimensions: dimensions,
            data: datavec,
        }
    }

    pub fn getslice(
        &self,
        location: Coordinate,
        orientation: AbsoluteDirection,
        offsets: &Vec<Coordinate>,
    ) -> Vec<Option<&WorldCell>> {
        let mut temp_vec = Vec::new();
        for i in 0..offsets.len() {
            temp_vec.push(self.get(&(location + offsets[i] * orientation)));
        }
        temp_vec
    }

    // Try to do this without clone() calls. Cannot move an object out of vec.
    pub fn setslice(
        &mut self,
        location: Coordinate,
        orientation: AbsoluteDirection,
        offsets: &Vec<Coordinate>,
        data: Vec<Option<WorldCell>>,
    ) -> bool {
        for i in 0..offsets.len() {
            self.set(&(location + offsets[i] * orientation), data[i].clone());
        }
        true
    }
}
