use crate::error::Result;

pub trait  Updatable {}

pub trait Delta {
    type Target: Updatable;
    fn new() -> Self;
    fn apply(&self, target: &mut Self::Target) -> Result<()>;
}

pub trait UpdatableContainer {
    type CoordinateType;
    type DataType: Clone;
    fn get(&self, key: &Self::CoordinateType) -> Result<&Self::DataType>;
    fn mut_set(&mut self, key: &Self::CoordinateType, value: &Self::DataType) -> Result<()>;
}

pub trait UpdatableContainerDelta {
    type CoordinateType;
    type DataType;
    type Target: UpdatableContainer<CoordinateType = Self::CoordinateType, DataType = Self::DataType>;

    fn get<'a>(&'a self, source: &'a Self::Target, key: &Self::CoordinateType) -> Result<&'a Self::DataType> {
        let cached = self.get_cached(key)?;
        match cached {
            Some(val) => Ok(val),
            None => {
                let real = source.get(key)?;
                Ok(real)
            }
        }
    }

    fn set(&mut self, key: &Self::CoordinateType, value: &Self::DataType) -> Result<()>;

    fn get_cached(&self, key: &Self::CoordinateType) -> Result<Option<&Self::DataType>>;
 }
