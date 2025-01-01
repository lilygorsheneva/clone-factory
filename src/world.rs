use crate::error::{Result, Status::StateUpdateError};
use crate::{
    actor::Actor,
    datatypes::{Building, Coordinate, Item},
    direction::AbsoluteDirection,
};

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

    pub fn set(&mut self, location: &Coordinate, data: Option<WorldCell>) -> Result<()> {
        match (data, self.in_bounds(&location)) {
            (None, false) => Ok(()),
            (None, true) => Err(StateUpdateError),
            (Some(_), false) => Err(StateUpdateError),
            (Some(cell), true) => {
                if let Some(new_state) = self.data.set((&self).coord_to_idx(&location), cell) {
                    self.data = new_state;
                    Ok(())
                } else {
                    Err(StateUpdateError)
                }
            }
        }
    }

    pub fn new(dimensions: Coordinate) -> World {
        let mut datavec = rpds::Vector::new();
        for _ in 0..(dimensions.x * dimensions.y) {
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
    ) ->  Result<()>  {
        for i in 0..offsets.len() {
            self.set(&(location + offsets[i] * orientation), data[i].clone())?;
        }
        Ok(())
    }
}
