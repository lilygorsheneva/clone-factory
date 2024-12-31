#[derive(Copy, Clone)]
pub enum AbsoluteDirection {
    N,
    S,
    E,
    W,
}

#[derive(Copy, Clone)]
pub enum RelativeDirection {
    F,
    B,
    L,
    R,
}

#[derive(Copy, Clone)]
pub enum Direction {
    Absolute(AbsoluteDirection),
    Relative(RelativeDirection),
}

impl Direction {
    fn to_int(&self) -> i8 {
        match self {
            Direction::Absolute(x) => x.to_int(),
            Direction::Relative(x) => x.to_int(),
        }
    }
}

impl RelativeDirection {
    fn to_int(&self) -> i8 {
        match self {
            RelativeDirection::F => 0,
            RelativeDirection::R => 1,
            RelativeDirection::B => 2,
            RelativeDirection::L => 3,
        }
    }
}

impl AbsoluteDirection {
    fn to_int(&self) -> i8 {
        match self {
            AbsoluteDirection::N => 0,
            AbsoluteDirection::E => 1,
            AbsoluteDirection::S => 2,
            AbsoluteDirection::W => 3,
        }
    }

    fn from_int(i: i8) -> AbsoluteDirection {
        match i % 4 {
            0 => AbsoluteDirection::N,
            1 => AbsoluteDirection::E,
            2 => AbsoluteDirection::S,
            3 => AbsoluteDirection::W,
            _ => panic!(),
        }
    }

    pub fn rotate(&self, update: &Direction) -> AbsoluteDirection {
        match update {
            Direction::Absolute(abs) => *abs,
            Direction::Relative(rel) => AbsoluteDirection::from_int(rel.to_int() + self.to_int()),
        }
    }
}

