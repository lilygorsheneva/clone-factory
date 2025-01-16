use crate::error::Result;

pub trait Updateable {
    type CoordinateType;
    type DataType;
    fn get(&self, key: &Self::CoordinateType) -> Result<Option<Self::DataType>>;
    fn mut_set(&mut self, key: &Self::CoordinateType, value: &Self::DataType) -> Result<()>;
}

pub trait Update
{
    type CoordinateType;
    type DataType;
    type UpdateTarget: Updateable<CoordinateType = Self::CoordinateType, DataType = Self::DataType>;

    fn get(&self, key: &Self::CoordinateType) -> Result<Option<Self::DataType>>;
    fn set(&mut self, key: &Self::CoordinateType, value: &Self::DataType) -> Result<()>;


    fn apply(&self, target: &mut Self::UpdateTarget) -> Result<()>;
}
