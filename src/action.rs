use crate::direction::Direction;
use crate::world::World;
use crate::datatypes::Actor;
use crate::datatypes::Coordinate;

pub struct Action {
    pub direction: Direction,
    pub action: SubAction
}

pub  enum SubAction {
    Move,
    Take,
    Drop,
    Use(u8),
    Craft(String),
    Record,
    EndRecording,
}
