use std::collections::{HashMap, HashSet};
use std::default;
use std::time::Instant;

use crate::error::Result;
use crate::{datatypes::Coordinate, error::Status};

use super::update::{Delta, Updatable, UpdatableContainer, UpdatableContainerDelta};
use super::worldlayer::{WorldLayer, WorldLayerDelta};

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct TrackableId(pub usize);

pub trait Trackable {
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
    recycle: HashSet<TrackableId>,
}

impl Index {
    fn new() -> Index {
        Index {
            data: HashMap::new(),
            recycle: HashSet::new(),
        }
    }

    fn get_next_id(&self) -> TrackableId {
        TrackableId(self.data.len() + self.recycle.len())
    }

    fn remove(&mut self, key: TrackableId) -> Result<()> {
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

#[derive(Debug)]
struct IndexDelta {
    mutates: HashMap<TrackableId, Coordinate>,
    inserts: Vec<TrackableId>,
    deletes: Vec<TrackableId>,
}

impl IndexDelta {
    fn get_next_id(&mut self, source: &Index) -> TrackableId {
        let ret = TrackableId(source.data.len() + source.recycle.len() + self.inserts.len());
        self.inserts.push(ret);
        ret
    }
    fn remove(&mut self, key: TrackableId) {
        self.deletes.push(key);
    }
}

impl Delta for IndexDelta {
    type Target = Index;
    fn new() -> Self {
        Self {
            mutates: HashMap::new(),
            inserts: Vec::new(),
            deletes: Vec::new(),
        }
    }

    fn apply(&self, target: &mut Self::Target) -> Result<()> {
        // TODO: validate. inserts and deletes do not intersect.

        for (k, v) in self.mutates.iter() {
            target.data.insert(*k, *v);
        }
        for k in &self.deletes {
            target.recycle.insert(*k);
        }
        Ok(())
    }
}

impl UpdatableContainerDelta for IndexDelta {
    type CoordinateType = TrackableId;
    type DataType = Coordinate;
    type Target = Index;

    fn get_cached(&self, key: &Self::CoordinateType) -> Result<Option<&Self::DataType>> {
        Ok(self.mutates.get(key))
    }

    fn set(&mut self, key: &Self::CoordinateType, value: &Self::DataType) -> Result<()> {
        self.mutates.insert(*key, *value);
        Ok(())
    }
}

pub struct TrackableWorldLayer<DataType: Clone + Trackable> {
    layer: WorldLayer<DataType>,
    index: Index,
}

impl<T: Clone + Trackable> TrackableWorldLayer<T> {
    pub fn new(dimensions: Coordinate, default: T) -> TrackableWorldLayer<T> {
        TrackableWorldLayer {
            layer: WorldLayer::new(dimensions, default),
            index: Index::new(),
        }
    }

    pub fn in_bounds(&self, location: &Coordinate) -> bool {
        self.layer.in_bounds(location)
    }
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

#[derive(Debug)]
pub struct TrackableWorldLayerDelta<DataType: Clone + Trackable> {
    layer: WorldLayerDelta<DataType>,
    index: IndexDelta,
}

impl<T: Clone + Trackable> TrackableWorldLayerDelta<T> {
    pub fn get_next_id(&mut self, source: &TrackableWorldLayer<T>) -> TrackableId {
        self.index.get_next_id(&source.index)
    }

    pub fn remove(&mut self, key: TrackableId) {
        self.index.remove(key);
    }
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
