use super::grid::Coordinate;

#[derive(Debug)]
pub enum Error {
    CellIndexOutOfRange(Coordinate),
    ValueOutOfRange(u8),
}
