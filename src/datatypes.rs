use crate::action::Action;
use crate::direction::AbsoluteDirection;
use crate::actor::Actor;
use std::ops;
use crate::db::RecordingId;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
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

#[derive(PartialEq, Debug)]
#[derive(Clone, Copy)]
pub struct Item {
    id: usize,
    quantity: u16,
    pub recording: Option<RecordingId>
}

impl Item {
    pub fn new(id: usize, quantity: u16) -> Item {
        Item {
            id: id,
            quantity: quantity,

            recording: None,
        }
    }

    pub fn new_cloner(recordingid: RecordingId) -> Item {
        Item {
            id: 0,
            quantity: 1,
            recording: Some(recordingid),
        }
    }
}

#[derive(PartialEq, Debug)]
#[derive(Clone)]
pub struct Building {
    name: String,
    facing: AbsoluteDirection,
}

#[derive(Clone)]
pub struct Recording {
    pub command_list: Vec<Action>,
    pub inventory: [Option<Item>; 5],
}

impl Recording {
    pub fn blank() -> Recording{
        Recording {
            command_list: Vec::new(),
            inventory: Default::default(),
        }
    }

    pub fn from_creator(actor: &Actor) -> Recording{
        Recording {
            command_list: Vec::new(),
            inventory: actor.inventory,
        }
    }

    pub fn at(&self, idx: usize) -> Action{
        self.command_list[idx % self.command_list.len()]
    }

    pub fn append(&mut self, action: Action) {
        self.command_list.push(action);
    }
}