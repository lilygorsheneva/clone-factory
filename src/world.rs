use crate::datatypes::{Actor, Building, Item, Coordinate};
use std::rc::Rc;

pub struct WorldCell {
    pub actor: Option<Actor>,
    pub building: Option<Building>,
    pub items: Rc<Vec<Item>>,
}

impl WorldCell {
    pub fn new() -> WorldCell {
        WorldCell {
            actor: None,
            building: None,
            items: Vec::new().into(),
        }
    }
}




pub struct World {
    pub dimensions: Coordinate,
    data: rpds::Vector<WorldCell>,
}

impl World {
    fn in_bounds(&self, location: &Coordinate) -> bool{
        location.x >= 0 && location.x < self.dimensions.x && location.y >= 0 && location.y < self.dimensions.y 
    }

    fn coord_to_idx(&self, location: &Coordinate) -> usize{
         (location.x + location.y * self.dimensions.x) as usize
    }

    pub fn get(&self, location: &Coordinate) -> Option<&WorldCell> {
        // In_rect misbehaves for some reason.
        if self.in_bounds(&location){
            return Some(&self.data[self.coord_to_idx(location)]);
        } else {
            return None
        }
    }

    pub fn init(dimensions: Coordinate) -> World {
        let mut datavec =rpds::Vector::new();
        for i in 0..(dimensions.x * dimensions.y) {
            datavec = datavec.push_back(WorldCell::new());
        }
        World{
            dimensions: dimensions,
            data: datavec
        }
    }

    pub fn spawn(&mut self, location: Coordinate, actor: Actor) -> bool {
        if !self.in_bounds(&location){
            return false
        } 
        let target = self.get(&location).unwrap();
        if target.actor.is_some() {
            return false
        }
        self.data = self.data.set(self.coord_to_idx(&location),WorldCell{
            actor:Some(actor),
            building: target.building.clone(),
            items: target.items.clone(),
        }).expect("Failed to update world");
        true
    }

}