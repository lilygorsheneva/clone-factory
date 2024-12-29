use std::{rc::Rc, vec};

pub struct Coordinate{
    pub x: i16,
    pub y: i16,
}

impl Coordinate {
    pub fn in_rect(&self, a: &Coordinate, b: &Coordinate) -> bool{
        self.x > a.x && self.x < b.x-1 && self.y > a.y && self.x < b.y-1
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

pub  enum AbsoluteDirection {
    N, S, E, W,
}

pub  enum RelativeDirection {
    F, B, L, R
}

pub  enum Direction {
    Absolute(AbsoluteDirection),
    Relative(RelativeDirection),
}

pub struct ActionQueue {
    q: Rc<Vec<Action>>
}

pub  enum Action {
    Move,
    Rotate(Direction),
    Take,
    Drop,
    Use(u8),
    Craft(String),
    Record,
    EndRecording,
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

pub struct WorldCell {
    pub actor: Option<Actor>,
    pub building: Option<Building>,
    pub items: Vec<Item>,
}

impl WorldCell {
    pub fn new() -> WorldCell {
        WorldCell {
            actor: None,
            building: None,
            items: Vec::new(),
        }
    }
}


pub struct World {
    pub dimensions: Coordinate,
    pub data: rpds::Vector<WorldCell>,
}

impl World {
    fn coord_to_idx(&self, location: Coordinate) -> usize{
         (location.x + location.y * self.dimensions.x) as usize
    }

    pub fn get(&self, location: Coordinate) -> Option<&WorldCell> {
        let zero =Coordinate::zero();
        if location.in_rect(&zero, &self.dimensions) {
            return Some(&self.data[self.coord_to_idx(location)]);
        } else {
            return None
        }
    }

    pub fn init(dimensions: Coordinate) -> World {
        let mut datavec =rpds::Vector::new();
        for i in 0..dimensions.x * dimensions.y {
            datavec = datavec.push_back(WorldCell::new());
        }
        let mut new_world = World{
            dimensions: dimensions,
            data: datavec};
        new_world
    }
}