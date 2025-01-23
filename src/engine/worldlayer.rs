use std::vec;

use std::collections::HashMap;

use super::update::{Delta, Updatable, UpdatableContainer, UpdatableContainerDelta};
use crate::datatypes::Coordinate;
use crate::error::Result;

#[derive(Debug)]
pub struct WorldLayer<DataType: Clone> {
    dimensions: Coordinate,
    pub data: Vec<DataType>,
}
impl<DataType: Clone> WorldLayer<DataType> {
    pub fn in_bounds(&self, location: &Coordinate) -> bool {
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

    pub fn new(dimensions: Coordinate, default: DataType) -> WorldLayer<DataType> {
        WorldLayer {
            dimensions: dimensions,
            data: vec![default; (dimensions.x * dimensions.y) as usize],
        }
    }
}

impl<DataType: Clone> Updatable for WorldLayer<DataType> {}

impl<DataType: Clone> UpdatableContainer for WorldLayer<DataType> {
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

#[derive(Debug)]
pub struct WorldLayerDelta<DataType: Clone> {
    writes: HashMap<Coordinate, DataType>,
}

impl<DataType: Clone> Delta for WorldLayerDelta<DataType> {
    type Target = WorldLayer<DataType>;
    fn new() -> Self {
        Self {
            writes: HashMap::new(),
        }
    }

    fn apply(&self, target: &mut WorldLayer<DataType>) -> Result<()> {
        for (k, v) in &self.writes {
            target.mut_set(k, v)?
        }
        Ok(())
    }
}

impl<DataType: Clone> UpdatableContainerDelta for WorldLayerDelta<DataType> {
    type CoordinateType = Coordinate;
    type DataType = DataType;
    type Target = WorldLayer<DataType>;

    fn set(&mut self, key: &Self::CoordinateType, value: &Self::DataType) -> Result<()> {
        self.writes.insert(key.clone(), value.clone());
        Ok(())
    }

    fn get_cached(&self, key: &Self::CoordinateType) -> Result<Option<&Self::DataType>> {
        Ok(self.writes.get(key))
    }
}
