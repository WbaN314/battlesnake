use crate::logic::game::field::BasicField;

#[derive(Clone, Copy, PartialEq)]
pub enum SituationField {
    OwnHead,
    OtherHead(u8), // 0=B, 1=C, 2=D
    MovableArea,
    NonMovableArea,
    Wall,
    Food,
    Any,
}

impl SituationField {
    pub fn from(c: char) -> Self {
        match c {
            'A' => Self::OwnHead,
            'B' => Self::OtherHead(0),
            'C' => Self::OtherHead(1),
            'D' => Self::OtherHead(2),
            '.' => Self::MovableArea,
            'N' => Self::NonMovableArea,
            'W' => Self::Wall,
            '*' => Self::Any,
            'X' => Self::Food,
            _ => panic!("Invalid character for SituationField: {}", c),
        }
    }

    pub fn display_char(&self) -> char {
        match self {
            Self::OwnHead => 'A',
            Self::OtherHead(0) => 'B',
            Self::OtherHead(1) => 'C',
            Self::OtherHead(_) => 'D',
            Self::MovableArea => '.',
            Self::NonMovableArea => 'N',
            Self::Wall => 'W',
            Self::Food => 'X',
            Self::Any => '*',
        }
    }

    pub fn check(&self, field: Option<BasicField>) -> bool {
        match field {
            None => matches!(self, Self::NonMovableArea | Self::Wall),
            Some(f) => match self {
                Self::OwnHead => matches!(f, BasicField::Snake { id: 0, next: None }),
                Self::OtherHead(_) => matches!(
                    f,
                    BasicField::Snake {
                        id: 1..=3,
                        next: None
                    }
                ),
                Self::MovableArea => matches!(f, BasicField::Empty | BasicField::Food),
                Self::Food => matches!(f, BasicField::Food),
                Self::NonMovableArea => matches!(f, BasicField::Snake { .. }),
                Self::Any | Self::Wall => true,
            },
        }
    }
}
