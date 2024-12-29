use crate::datatypes::{Actor, Building, Item, Coordinate};

pub struct WorldCell {
    pub actor: Option<Actor>,
    pub building: Option<Building>,
    pub items: Vec<Item>,
}

impl WorldCell {
    pub fn new() -> WorldCell {
        WorldCell {
            actor: None,
            building: None,
            items: Vec::new(),
        }
    }
}


pub struct World {
    pub dimensions: Coordinate,
    data: rpds::Vector<WorldCell>,
}

impl World {
    fn coord_to_idx(&self, location: Coordinate) -> usize{
         (location.x + location.y * self.dimensions.x) as usize
    }

    pub fn get(&self, location: Coordinate) -> Option<&WorldCell> {
        // In_rect misbehaves for some reason.
        if location.x >= 0 && location.x < self.dimensions.x && location.y >= 0 && location.y < self.dimensions.y {
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
}