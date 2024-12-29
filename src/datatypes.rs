use std::rc::Rc;
use crate::action::Action;
use crate::direction::AbsoluteDirection;

pub struct Coordinate{
    pub x: i16,
    pub y: i16,
}

impl Coordinate {
    pub fn in_rect(&self, a: &Coordinate, b: &Coordinate) -> bool{
        (self.x >= a.x && self.x < b.x && self.y >= a.y && self.x < b.y)
    }

    pub fn zero() -> Coordinate{Coordinate {x:0,y:0}}
}

pub  struct Item {
    name: String,
    quantity: u16,

    // Tags
    ephemeral: bool,
    cloneable: bool,
    physical: bool,
    bound: bool,
    // End tags
    recording: Rc<Recording>
}

pub  struct Building {
    name: String,
    facing: AbsoluteDirection,
}

pub struct ActionQueue {
    q: Rc<Vec<Action>>
}

pub  struct Recording {
    command_list: ActionQueue,
    equipment: Rc<Vec<Item>>,
}


// A recording will probably be a partially-defined actor.
pub  struct Actor {
    location: Coordinate,
    facing: AbsoluteDirection,
    isplayer: bool,
    command_list: ActionQueue,
    command_idx: u8,
    inventory: Rc<Vec<Item>>,
    equipment: Rc<Vec<Item>>,
}