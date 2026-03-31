use std::char;

use crate::logic::game::field::BasicField;

pub enum SituationField {
    OwnHead,
    OtherHead,
    MovableArea,
    NonMovableArea,
    Any,
}
impl SituationField {
    pub fn from(char: char) -> Self {
        match char {
            'A' => SituationField::OwnHead,
            'B' | 'C' | 'D' => SituationField::OtherHead,
            '.' => SituationField::MovableArea,
            'N' => SituationField::NonMovableArea,
            '*' => SituationField::Any,
            _ => panic!("Invalid character for SituationField: {}", char),
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
