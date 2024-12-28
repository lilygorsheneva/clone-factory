use std::rc::Rc;

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
    command_list: Vec<Action>,
    equipment: Vec<Item>,
}


// A recording will probably be a partially-defined actor.
pub  struct Actor {
    location: Coordinate,
    facing: AbsoluteDirection,
    isplayer: bool,
    command_list: Vec<Action>,
    inventory: Vec<Item>,
    equipment: Vec<Item>,
}

pub struct WorldCell {
    pub actor: Option<Actor>,
    pub building: Option<Building>,
    pub items: Vec<Item>,
}

pub struct World {
    pub dimensions: Coordinate,
    pub data: Vec<Vec<WorldCell>>
}