#[derive(Debug,PartialEq)]
#[derive(Copy, Clone)]
pub enum AbsoluteDirection {
    N,
    S,
    E,
    W,
}
#[derive(Debug,PartialEq)]
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum RelativeDirection {
    F,
    B,
    L,
    R,
}

#[derive(Debug,PartialEq)]
#[derive(Copy, Clone)]
pub enum Direction {
    Absolute(AbsoluteDirection),
    Relative(RelativeDirection),
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
            _ => unreachable!(),
        }
    }

    pub fn rotate(&self, update: &Direction) -> AbsoluteDirection {
        match update {
            Direction::Absolute(abs) => *abs,
            Direction::Relative(rel) => AbsoluteDirection::from_int(rel.to_int() + self.to_int()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_relative() {
        let facing = AbsoluteDirection::N;
        assert_eq!(
            AbsoluteDirection::E,
            facing.rotate(&Direction::Relative(RelativeDirection::R))
        )
    }

    #[test]
    fn rotate_absolute() {
        let facing = AbsoluteDirection::N;
        assert_eq!(
            AbsoluteDirection::E,
            facing.rotate(&Direction::Absolute(AbsoluteDirection::E))
        )
    }
}
