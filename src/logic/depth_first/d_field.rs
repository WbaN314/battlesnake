use super::d_direction::DDirection;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DField {
    Empty,
    Food,
    Snake {
        id: u8,
        stack: u8,
        next: Option<DDirection>,
    },
}

impl Default for DField {
    fn default() -> Self {
        DField::Empty
    }
}
