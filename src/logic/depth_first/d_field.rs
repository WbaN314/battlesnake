#[derive(Clone, Copy)]
pub enum DField {
    Empty,
    Food,
}

impl Default for DField {
    fn default() -> Self {
        DField::Empty
    }
}
