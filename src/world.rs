use crate::error::{Result, Status::StateUpdateError};
use crate::{
    actor::Actor,
    datatypes::{Building, Coordinate},
    direction::AbsoluteDirection,
};
use crate::inventory::Item;

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

pub struct World {
    pub dimensions: Coordinate,
    data: rpds::Vector<WorldCell>,
}

#[derive(Debug)]
pub struct WorldUpdate {
    map: HashMap<Coordinate, WorldCell>,
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

    fn read<'a>(
        &self,
        update: &'a mut WorldUpdate,
        location: &Coordinate,
    ) -> Option<&'a mut WorldCell> {
        match (self.in_bounds(location), update.map.contains_key(location)) {
            (true, false) => {
                update
                    .map
                    .insert(*location, self.data[self.coord_to_idx(location)].clone());
                update.map.get_mut(location)
            }
            (true, true) => update.map.get_mut(location),
            (false, _) => None,
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

    pub fn readslice<'a, const N: usize>(
        &self,
        update: &'a mut WorldUpdate,
        location: Coordinate,
        orientation: AbsoluteDirection,
        offsets: &[Coordinate; N],
    ) -> [Option<&'a mut WorldCell>; N] {
        let translated_offsets = offsets.map(|offset| location +  offset * orientation);

        for offset in translated_offsets {
            self.read(update, &offset);
        }
        update.map.get_many_mut(translated_offsets.each_ref())
    }

    pub fn new_update(&self) -> WorldUpdate {
        WorldUpdate {
            map: HashMap::new(),
        }
    }

    pub fn apply_update(&mut self, update: &WorldUpdate) -> Result<()> {
        for (coordinate, cell) in &update.map {
            self.mut_set(&coordinate, Some(cell.clone()))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let w = World::new(Coordinate { x: 1, y: 1 });
        assert!(w.in_bounds(&Coordinate { x: 0, y: 0 }));
        assert!(w.get(&Coordinate { x: 0, y: 0 }).is_some());
    }

    #[test]
    fn mutate() {
        let mut w = World::new(Coordinate { x: 1, y: 1 });
        let location = Coordinate { x: 0, y: 0 };
        let oldcell = w.get(&location).unwrap();
        let newcell = WorldCell {
            actor: Some(Actor::new()),
            ..oldcell.clone()
        };
        assert_ne!(*oldcell, newcell); // Sanity check to ensure we actually mutate.
        w.mut_set(&location, Some(newcell.clone())).unwrap();
        assert_eq!(*w.get(&Coordinate { x: 0, y: 0 }).unwrap(), newcell);
    }

    #[test]
    fn update() {
        let mut w = World::new(Coordinate { x: 1, y: 1 });
        let location = Coordinate { x: 0, y: 0 };
        let orientation = AbsoluteDirection::N;
        let offsets = [Coordinate { x: 0, y: 0 }];
        let mut update = w.new_update();

        let mut reads = w.readslice(&mut update, location, orientation, &offsets);

        if let Some(cell) = &mut reads[0] {
            assert!(cell.actor.is_none());
            cell.actor = Some(Actor::new());

            assert!(w.apply_update(&update).is_ok());
            assert!(w.get(&Coordinate { x: 0, y: 0 }).unwrap().actor.is_some());
        }
    }
}
