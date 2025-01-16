use crate::error::Result;

pub trait Updatable {
    type CoordinateType;
    type DataType: Clone;
    fn get(&self, key: &Self::CoordinateType) -> Result<&Self::DataType>;
    fn mut_set(&mut self, key: &Self::CoordinateType, value: &Self::DataType) -> Result<()>;
}

pub trait Update {
    type CoordinateType;
    type DataType;
    type UpdateTarget: Updatable<CoordinateType = Self::CoordinateType, DataType = Self::DataType>;

    fn new() -> Self;

    fn get<'a>(&'a self, source: &'a Self::UpdateTarget, key: &Self::CoordinateType) -> Result<&'a Self::DataType> {
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

    // Call mut_set in a loop. Needs some sort of Iterator that I don't know how to define yet.
    fn apply(&self, target: &mut Self::UpdateTarget) -> Result<()>;
}
