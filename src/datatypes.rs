//! Temporary miscellaneous datatype container.
//! Any structure that increases in complexity should be moved to its own file.

use crate::direction::AbsoluteDirection;
use std::{cmp::{max, min}, ops};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn as_offset(offset: Coordinate, location: Coordinate, orientation: AbsoluteDirection) -> Coordinate {
        location + offset*orientation
    } 

    pub fn clamp(self, v1: Coordinate, v2: Coordinate)->Coordinate{
        let (minx, maxx) = (min(v1.x,v2.x),max(v1.x,v2.x));
        let (miny, maxy) = (min(v1.y,v2.y),max(v1.y,v2.y));
        Coordinate {
            x: self.x.clamp(minx, maxx),
            y: self.y.clamp(miny, maxy),
        }
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

// North, Up and +y are synonyms
// East, Right, and +x are synonyms
// Default facing direction for actions is North. 
impl ops::Mul<AbsoluteDirection> for Coordinate {
    type Output = Coordinate;

    fn mul(self, _rhs: AbsoluteDirection) -> Coordinate {
        match _rhs {
            AbsoluteDirection::N => Coordinate {
                x: self.x,
                y: self.y,
            },
            AbsoluteDirection::E => Coordinate {
                x: self.y,
                y: -self.x,
            },
            AbsoluteDirection::S => Coordinate {
                x: -self.x,
                y: -self.y,
            },
            AbsoluteDirection::W => Coordinate {
                x: -self.y,
                y: self.x,
            },
        }
    }
}
