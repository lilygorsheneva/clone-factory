//! Temporary miscellaneous datatype container.
//! Any structure that increases in complexity should be moved to its own file.

use crate::action::Action;
use crate::actor::Actor;
use crate::direction::AbsoluteDirection;
use crate::inventory::BasicInventory;
use std::ops;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Coordinate {
    pub x: i16,
    pub y: i16,
}

impl Coordinate {
    pub fn as_offset(offset: Coordinate, location: Coordinate, orientation: AbsoluteDirection) -> Coordinate {
        location + offset*orientation
    } 
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

#[derive(PartialEq, Debug, Clone)]
pub struct Building {
    name: String,
    facing: AbsoluteDirection,
}
