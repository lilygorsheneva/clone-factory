use std::rc::Rc;
use std::vec;
use crate::action::Action;
use crate::direction::AbsoluteDirection;

pub struct Coordinate{
    pub x: i16,
    pub y: i16,
}

impl Coordinate {
    pub fn new(x:i16, y:i16) -> Coordinate {
        Coordinate{x:x, y:y}
    }

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

#[derive(Clone)]
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
    facing: AbsoluteDirection,
    isplayer: bool,
    command_list: ActionQueue,
    command_idx: u8,
    inventory: Rc<Vec<Item>>,
    equipment: Rc<Vec<Item>>,
}

impl Actor {
    pub fn new() -> Actor{
        Actor {
            facing: AbsoluteDirection::N,
            isplayer: false,
            command_list: ActionQueue{q: Vec::new().into()},
            command_idx: 0,
            inventory: Vec::new().into(),
            equipment: Vec::new().into()            
        }
    }
}