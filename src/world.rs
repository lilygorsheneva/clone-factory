use crate::error::{Result, Status::StateUpdateError};
use crate::{
    actor::Actor,
    datatypes::{Building, Coordinate, Item},
    direction::AbsoluteDirection,
};
use std::collections::HashSet;

#[derive(PartialEq, Debug)]
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

#[derive(Debug)]
pub struct WorldUpdate {
cells: Vec<(Coordinate, Option<WorldCell>)>
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

    pub fn mut_set(&mut self, location: &Coordinate, data: Option<WorldCell>) -> Result<()> {
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
    pub fn mut_setslice(
        &mut self,
        location: Coordinate,
        orientation: AbsoluteDirection,
        offsets: &Vec<Coordinate>,
        data: Vec<Option<WorldCell>>,
    ) ->  Result<()>  {
        for i in 0..offsets.len() {
            self.mut_set(&(location + offsets[i] * orientation), data[i].clone())?;
        }
        Ok(())
    }

    pub fn new_update(&self) -> WorldUpdate {
        WorldUpdate {cells:Vec::new()}
    }

    // Try to do this without clone() calls. Cannot move an object out of vec.
    pub fn update_slice(&self,
        update: &mut WorldUpdate,
        location: Coordinate,
        orientation: AbsoluteDirection,
        offsets: &Vec<Coordinate>,
        data: Vec<Option<WorldCell>>) ->Result<()> {
            for i in 0..offsets.len() {
                update.cells.push((location + offsets[i] * orientation, data[i].clone()));
            }
            Ok(())
        }

    pub fn apply_update(&mut self, update: &WorldUpdate) -> Result<()> {
        let mut coord_set: HashSet<Coordinate> = HashSet::new();

        for (coordinate, cell) in &update.cells {
            match coord_set.insert(*coordinate) {
                false => return Err(StateUpdateError),
                true => self.mut_set(&coordinate, cell.clone())?
            }
        }

        Ok(())
}
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let w = World::new(Coordinate{x:1, y:1});
        assert!(w.in_bounds(&Coordinate{x:0, y:0}));
        assert!(w.get(&Coordinate{x:0, y:0}).is_some());
    }
    
    #[test]
    fn mutate() {
        let mut w = World::new(Coordinate{x:1, y:1});
        let location = Coordinate{x:0, y:0};
        let oldcell = w.get(&location).unwrap();
        let newcell = WorldCell {
            actor: Some(Actor::new()),
            ..oldcell.clone()
        };
        assert_ne!(*oldcell, newcell); // Sanity check to ensure we actually mutate.
        w.mut_set(&location, Some(newcell.clone())).unwrap();
        assert_eq!(*w.get(&Coordinate{x:0, y:0}).unwrap(), newcell);
    }

    #[test]
    fn update() {
        let mut w = World::new(Coordinate{x:1, y:1});
        let location = Coordinate{x:0, y:0};
        let orientation = AbsoluteDirection::N;
        let offsets = vec![Coordinate{x:0, y:0}];

        let oldcell = w.getslice(location, AbsoluteDirection::N, &offsets)[0].unwrap();
        let newcell = WorldCell {
            actor: Some(Actor::new()),
            ..oldcell.clone()
        };
        assert_ne!(*oldcell, newcell); // Sanity check to ensure we actually mutate.

        let mut update = w.new_update();
        let res = w.update_slice(&mut update, location, orientation, &offsets, vec![Some(newcell.clone())]);
        assert!(w.apply_update(&update).is_ok());
        assert!(res.is_ok());
        assert_eq!(*w.get(&Coordinate{x:0, y:0}).unwrap(), newcell);
    }

    #[test]
    fn update_reject_overlap() {
        let mut w = World::new(Coordinate{x:1, y:1});
        let location = Coordinate{x:0, y:0};
        let orientation = AbsoluteDirection::N;
        let offsets = vec![Coordinate{x:0, y:0}];

        let newcell = WorldCell::new();

        let mut update = w.new_update();
        let _ = w.update_slice(&mut update, location, orientation, &offsets, vec![Some(newcell.clone())]);
        let _ = w.update_slice(&mut update, location, orientation, &offsets, vec![Some(newcell.clone())]);

        assert!(w.apply_update(&update).is_err());
    }

}