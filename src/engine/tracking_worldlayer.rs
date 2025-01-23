use std::collections::{HashMap, HashSet};

use crate::error::Result;
use crate::{datatypes::Coordinate, error::Status};

use super::update::{Delta, Updatable, UpdatableContainer, UpdatableContainerDelta};
use super::worldlayer::{WorldLayer, WorldLayerDelta};

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct TrackableId(usize);

trait Trackable {
    fn get_id(&self) -> Option<TrackableId>;
}

impl<T: Trackable> Trackable for Option<T> {
    fn get_id(&self) -> Option<TrackableId> {
        match self {
            None => None,
            Some(foo) => foo.get_id(),
        }
    }
}

struct Index {
    data: HashMap<TrackableId, Coordinate>,
    recycle: HashSet<TrackableId>
}

impl Index {
    fn get_next_id(&self ) -> TrackableId {
        TrackableId(self.data.len() + self.recycle.len())
    }

    fn remove(&mut self, key: TrackableId) -> Result<()>{
        self.data.remove(&key).ok_or(Status::OutOfBounds)?;
        self.recycle.insert(key);
        Ok(())
    }
}

impl Updatable for Index {}
impl UpdatableContainer for Index {
    type CoordinateType = TrackableId;
    type DataType = Coordinate;

    fn get(&self, key: &Self::CoordinateType) -> Result<&Self::DataType> {
        self.data.get(key).ok_or(Status::OutOfBounds)
    }
    fn mut_set(&mut self, key: &Self::CoordinateType, value: &Self::DataType) -> Result<()> {
        self.data.insert(*key, *value);
        self.recycle.remove(key);
        Ok(())
    }
}

struct IndexDelta {
    data: HashMap<TrackableId, Coordinate>,
    recycle: Vec<TrackableId>
}

impl Delta for IndexDelta {
    type Target = Index;
    fn new() -> Self {
        Self {
            data: HashMap::new(),
            recycle: Vec::new()
        }
    }

    fn apply(&self, target: &mut Self::Target) -> Result<()> {
        for (k, v) in self.data.iter() {
            target.data.insert(*k, *v);
        }
        Ok(())
    }
}

impl UpdatableContainerDelta for IndexDelta {
    type CoordinateType = TrackableId;
    type DataType = Coordinate;
    type Target = Index;

    fn get_cached(&self, key: &Self::CoordinateType) -> Result<Option<&Self::DataType>> {
        Ok(self.data.get(key))
    }

    fn set(&mut self, key: &Self::CoordinateType, value: &Self::DataType) -> Result<()> {
        self.data.insert(*key, *value);
        Ok(())
    }
}

struct TrackableWorldLayer<DataType: Clone + Trackable> {
    layer: WorldLayer<DataType>,
    index: Index,
}

impl<T: Clone + Trackable> Updatable for TrackableWorldLayer<T> {}

impl<T: Clone + Trackable> UpdatableContainer for TrackableWorldLayer<T> {
    type CoordinateType = Coordinate;
    type DataType = T;

    fn get(&self, key: &Self::CoordinateType) -> Result<&Self::DataType> {
        self.layer.get(key)
    }
    fn mut_set(
        &mut self,
        coordinate: &Self::CoordinateType,
        object: &Self::DataType,
    ) -> Result<()> {
        self.layer.mut_set(coordinate, object)?;
        if let Some(id) = object.get_id() {
            self.index.mut_set(&id, coordinate)?;
        }
        Ok(())
    }
}

struct TrackableWorldLayerDelta<DataType: Clone + Trackable> {
    layer: WorldLayerDelta<DataType>,
    index: IndexDelta,
}

impl<T: Clone + Trackable> Delta for TrackableWorldLayerDelta<T> {
    type Target = TrackableWorldLayer<T>;
    fn new() -> Self {
        Self {
            layer: WorldLayerDelta::new(),
            index: IndexDelta::new(),
        }
    }

    fn apply(&self, target: &mut Self::Target) -> Result<()> {
        self.layer.apply(&mut target.layer)?;
        self.index.apply(&mut target.index)?;
        Ok(())
    }
}

impl<T: Clone + Trackable> UpdatableContainerDelta for TrackableWorldLayerDelta<T> {
    type CoordinateType = Coordinate;
    type DataType = T;
    type Target = TrackableWorldLayer<T>;

    fn get_cached(&self, key: &Self::CoordinateType) -> Result<Option<&Self::DataType>> {
        self.layer.get_cached(key)
    }

    fn set(&mut self, coordinate: &Self::CoordinateType, object: &Self::DataType) -> Result<()> {
        self.layer.set(coordinate, object)?;
        if let Some(id) = object.get_id() {
            self.index.set(&id, coordinate)?;
        }
        Ok(())
    }
}
