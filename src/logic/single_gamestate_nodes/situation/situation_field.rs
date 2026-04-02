use crate::logic::game::field::BasicField;

#[derive(Clone, Copy, PartialEq)]
pub enum SituationField {
    OwnHead,
    OtherHead,
    MovableArea,
    NonMovableArea,
    Any,
}

impl SituationField {
    pub fn from(c: char) -> Self {
        match c {
            'A' => Self::OwnHead,
            'B' => Self::OtherHead,
            '.' => Self::MovableArea,
            'N' => Self::NonMovableArea,
            '*' => Self::Any,
            _ => panic!("Invalid character for SituationField: {}", c),
        }
    }

    pub fn display_char(&self) -> char {
        match self {
            Self::OwnHead        => 'A',
            Self::OtherHead      => 'B',
            Self::MovableArea    => '.',
            Self::NonMovableArea => 'N',
            Self::Any            => '*',
        }
    }

    pub fn check(&self, field: Option<BasicField>) -> bool {
        match field {
            None => matches!(self, Self::NonMovableArea),
            Some(f) => match self {
                Self::OwnHead => matches!(f, BasicField::Snake { id: 0, next: None }),
                Self::OtherHead => matches!(
                    f,
                    BasicField::Snake {
                        id: 1..=3,
                        next: None
                    }
                ),
                Self::MovableArea => matches!(f, BasicField::Empty | BasicField::Food),
                Self::NonMovableArea => matches!(f, BasicField::Snake { .. }),
                Self::Any => true,
            },
        }
    }
}
