use std::{default, vec};

use hashbrown::HashMap;

use super::update::{Update, Updatable};
use crate::datatypes::Coordinate;
use crate::error::Result;

pub struct WorldLayer<DataType: Clone> {
    dimensions: Coordinate,
    pub data: Vec<DataType>,
}
impl<DataType: Clone> WorldLayer<DataType> {
    fn in_bounds(&self, location: &Coordinate) -> bool {
        location.x >= 0
            && location.x < self.dimensions.x
            && location.y >= 0
            && location.y < self.dimensions.y
    }

    fn coord_to_idx(&self, location: &Coordinate) -> Result<usize> {
        if self.in_bounds(location) {
            Ok((location.x + location.y * self.dimensions.x) as usize)
        } else {
            Err(crate::error::Status::OutOfBounds)
        }
    }

    fn new(dimensions: Coordinate, default: DataType) -> WorldLayer<DataType> {
        WorldLayer {
            dimensions: dimensions,
            data: vec![default; (dimensions.x * dimensions.y) as usize],
        }
    }
}

impl<DataType: Clone> Updatable for WorldLayer<DataType> {
    type CoordinateType = Coordinate;
    type DataType = DataType;
    fn get(&self, key: &Self::CoordinateType) -> Result<&DataType> {
        let idx = self.coord_to_idx(key)?;
        Ok(&self.data[idx])
    }
    fn mut_set(&mut self, key: &Self::CoordinateType, value: &Self::DataType) -> Result<()> {
        let idx = self.coord_to_idx(key)?;
        self.data[idx] = value.clone();
        Ok(())
    }
}

pub struct WorldLayerUpdate<'a, DataType:Clone> {
    source: Option<&'a WorldLayer<DataType>>,
    writes: HashMap<Coordinate, DataType>,
}

impl<'a, DataType:Clone> Update<'a> for WorldLayerUpdate<'a, DataType> {
    type CoordinateType = Coordinate;
    type DataType = DataType;
    type UpdateTarget = WorldLayer<DataType>;

    fn new(source: &'a WorldLayer<DataType>) -> Self {
        Self {
            source: Some(source),
            writes: HashMap::new()
        }
    }

    fn source(&self) -> &'a Self::UpdateTarget {
        self.source.expect("Attempting to read through WorldLayerUpdate that has been written")
    }


    fn set(&mut self, key: &Self::CoordinateType, value: &Self::DataType) -> Result<()> {
        self.writes.insert(key.clone(), value.clone());
        Ok(())
    }

    fn get_cached(&self, key: &Self::CoordinateType) -> Result<Option<&Self::DataType>> {
        Ok(self.writes.get(key))
    }

    // Call mut_set in a loop. Needs some sort of Iterator that I don't know how to define yet.
    fn apply(&mut self, target: &mut WorldLayer<DataType>) -> Result<()> {
        self.source = None;
        for (k, v) in &self.writes {
            target.mut_set(k, v)?
        }
        Ok(())
    }
} 
