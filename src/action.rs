use crate::direction::Direction;

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
