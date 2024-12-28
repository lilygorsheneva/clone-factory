use std::rc::Rc;

type Coordinate = i16;

struct Item {
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

struct Building {
    name: String,
    facing: AbsoluteDirection,
}

enum AbsoluteDirection {
    N, S, E, W,
}

enum RelativeDirection {
    F, B, L, R
}

enum Direction {
    Absolute(AbsoluteDirection),
    Relative(RelativeDirection),
}

enum Action {
    Move,
    Rotate(Direction),
    Take,
    Drop,
    Use(u8),
    Craft(String),
    Record,
    EndRecording,
}


struct Recording {
    command_list: Vec<Action>,
    equipment: Vec<Item>,
}


// A recording will probably be a partially-defined actor.
struct Actor {
    x: Coordinate,
    y: Coordinate,
    facing: AbsoluteDirection,
    isplayer: bool,
    command_list: Vec<Action>,
    inventory: Vec<Item>,
    equipment: Vec<Item>,
}

struct WorldCell {
    actor: Option<Actor>,
    building: Option<Building>,
    items: Vec<Item>,
    ground: bool,
}


struct World {
    length: Coordinate,
    width: Coordinate,
    data: Vec<Vec<WorldCell>>
}