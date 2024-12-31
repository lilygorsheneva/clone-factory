use crate::action::Action;
use crate::direction::AbsoluteDirection;
use std::ops;
use std::process::Command;
use crate::db::RecordingId;

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
        self.x >= a.x && self.x < b.x && self.y >= a.y && self.x < b.y
    }

    pub const ZERO: Coordinate = Coordinate { x: 0, y: 0 };
    
}

#[derive(Clone, Copy)]
pub struct Item {
    id: usize,
    quantity: u16,

    // Tags
    ephemeral: bool,
    cloneable: bool,
    physical: bool,
    bound: bool,
    // End tags
    pub recording: Option<RecordingId>
}

impl Item {
    pub fn new(id: usize, quantity: u16) -> Item {
        Item {
            id: id,
            quantity: quantity,
            // Tags
            ephemeral: false,
            cloneable: false,
            physical: false,
            bound: false,
            // End tags
            recording: None,
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
    pub command_list: ActionQueue,
    pub equipment: Vec<Item>,
}

impl Recording {
    pub fn blank() -> Recording{
        Recording {
            command_list: ActionQueue {q: Vec::new()},
            equipment: Vec::new()
        }
    }
}