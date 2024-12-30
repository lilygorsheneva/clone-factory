use crate::action::Action;
use crate::direction::AbsoluteDirection;
use std::ops;

#[derive(Clone, Copy)]
pub struct Coordinate {
    pub x: i16,
    pub y: i16,
}

impl ops::Add<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn add(self, _rhs: Coordinate) -> Coordinate {
        Coordinate {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl ops::Mul<AbsoluteDirection> for Coordinate {
    type Output = Coordinate;

    fn mul(self, _rhs: AbsoluteDirection) -> Coordinate {
        match _rhs {
            AbsoluteDirection::N => Coordinate {
                x: self.x,
                y: self.y,
            },
            AbsoluteDirection::E => Coordinate {
                x: -self.y,
                y: self.x,
            },
            AbsoluteDirection::S => Coordinate {
                x: -self.x,
                y: -self.y,
            },
            AbsoluteDirection::W => Coordinate {
                x: self.y,
                y: -self.x,
            },
        }
    }
}

impl Coordinate {
    pub fn in_rect(&self, a: &Coordinate, b: &Coordinate) -> bool {
        (self.x >= a.x && self.x < b.x && self.y >= a.y && self.x < b.y)
    }

    pub const ZERO: Coordinate = Coordinate { x: 0, y: 0 };
    
}

#[derive(Clone)]
pub struct Item {
    name: String,
    quantity: u16,

    // Tags
    ephemeral: bool,
    cloneable: bool,
    physical: bool,
    bound: bool,
    // End tags
    // recording: Rc<Recording>
}

impl Item {
    pub fn new(name: String, quantity: u16) -> Item {
        Item {
            name: name,
            quantity: quantity,
            // Tags
            ephemeral: false,
            cloneable: false,
            physical: false,
            bound: false,
            // End tags
        }
    }
}

#[derive(Clone)]
pub struct Building {
    name: String,
    facing: AbsoluteDirection,
}

pub struct ActionQueue {
    pub q: Vec<Action>,
}

pub struct Recording {
    command_list: ActionQueue,
    equipment: Vec<Item>,
}
