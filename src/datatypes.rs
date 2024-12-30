use std::rc::Rc;
use crate::action::Action;
use crate::direction::AbsoluteDirection;
use std::ops;

#[derive(Clone, Copy)]
pub struct Coordinate{
    pub x: i16,
    pub y: i16,
}

impl ops::Add<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn add(self, _rhs: Coordinate) -> Coordinate {
        Coordinate {
            x:self.x + _rhs.x,
            y:self.y + _rhs.y
        }
    }
}

impl ops::Mul<AbsoluteDirection> for Coordinate {
    type Output = Coordinate;

    fn mul(self, _rhs: AbsoluteDirection) -> Coordinate {
        match _rhs {
            AbsoluteDirection::N=> Coordinate{x:self.x, y:self.y},
            AbsoluteDirection::E=> Coordinate{x:-self.y, y:self.x},
            AbsoluteDirection::S=> Coordinate{x:-self.x, y:-self.y},
            AbsoluteDirection::W=> Coordinate{x:self.y, y:-self.x},
        }  
    }
}

impl Coordinate {

    pub fn in_rect(&self, a: &Coordinate, b: &Coordinate) -> bool{
        (self.x >= a.x && self.x < b.x && self.y >= a.y && self.x < b.y)
    }

    pub fn zero() -> Coordinate{Coordinate {x:0,y:0}}
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct ActionQueue {
    q: Rc<Vec<Action>>
}

pub  struct Recording {
    command_list: ActionQueue,
    equipment: Rc<Vec<Item>>,
}


// A recording will probably be a partially-defined actor.
#[derive(Clone)]
pub  struct Actor {
    pub facing: AbsoluteDirection,
    isplayer: bool,
    command_list: ActionQueue,
    command_idx: usize,
    inventory: Rc<Vec<Item>>,
    equipment: Rc<Vec<Item>>,
}

impl Actor {
    pub fn get_action(&self) -> &Action{
    match self.isplayer{
        false => &self.command_list.q[self.command_idx],
        true =>  panic!("Called get_action on player entity.")
        }
    }
}

pub struct ActorRef {
    pub location: Coordinate
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