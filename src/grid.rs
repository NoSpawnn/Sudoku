use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum Error {
    CellIndexOutOfRange(Coordinate),
    ValueOutOfRange(u8),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Coordinate {
    row: usize,
    col: usize,
}

pub struct Grid {
    pub cells: Vec<Cell>,
}

impl Grid {
    const SUBGRID_ROWS: usize = 3;
    const SUBGRID_COLS: usize = 3;
    const ROW_COUNT: usize = Self::SUBGRID_ROWS * 3;
    const COL_COUNT: usize = Self::SUBGRID_ROWS * 3;
    const CELL_COUNT: usize = Self::ROW_COUNT * Self::COL_COUNT;
    const MIN_CELL_VALUE: u8 = 0;
    const MAX_CELL_VALUE: u8 = 9;

    pub fn new_empty() -> Self {
        Self {
            cells: vec![Cell::Empty; Self::CELL_COUNT],
        }
    }

    pub fn new_random() -> Self {
        let mut cells = Vec::with_capacity(Self::CELL_COUNT);

        for row in 0..Self::ROW_COUNT {
            for col in 0..Self::COL_COUNT {
                // ??
            }
        }

        Self { cells }
    }

    pub fn get_subgrid_start(c: Coordinate) -> Result<Coordinate, Error> {
        if c.row >= Self::ROW_COUNT || c.col >= Self::COL_COUNT {
            return Err(Error::CellIndexOutOfRange(c));
        }

        Ok(Coordinate {
            row: c.row / Self::SUBGRID_ROWS * Self::SUBGRID_ROWS,
            col: c.col / Self::SUBGRID_COLS * Self::SUBGRID_COLS,
        })
    }

    pub fn set_cell(&mut self, c: Coordinate, value: u8) -> Result<(), Error> {
        if c.row >= Self::ROW_COUNT || c.col >= Self::COL_COUNT {
            return Err(Error::CellIndexOutOfRange(c));
        } else if value <= Self::MIN_CELL_VALUE || value > Self::MAX_CELL_VALUE {
            return Err(Error::ValueOutOfRange(value));
        }

        self.set_cell_unchecked(c, value);

        Ok(())
    }

    fn set_cell_unchecked(&mut self, c: Coordinate, value: u8) {
        let idx = c.row * Self::COL_COUNT + c.col;
        self.cells[idx] = Cell::Filled(value);
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        write!(f, " ")?;
        for i in 1..=Self::COL_COUNT {
            write!(f, "{i:4}")?;
        }

        for (i, cell) in self.cells.iter().enumerate() {
            if i % Self::COL_COUNT == 0 {
                writeln!(f)?;
                writeln!(f)?;
                write!(f, "{}  ", i / Self::COL_COUNT + 1)?;
            }

            match cell {
                Cell::Filled(value) => write!(f, "[{value}] ")?,
                Cell::Empty => write!(f, "[ ] ")?,
            }
        }

        Ok(())
    }
}

#[derive(Default, Copy, Clone)]
pub enum Cell {
    #[default]
    Empty,
    Filled(u8),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_subgrid_start() {
        let tests: Vec<((usize, usize), (usize, usize))> = vec![
            ((0, 0), (0, 0)),
            ((1, 1), (0, 0)),
            ((4, 4), (3, 3)),
            ((8, 8), (6, 6)),
        ];

        for (c, e) in tests {
            let c = Coordinate { row: c.0, col: c.1 };
            let expected = Coordinate { row: e.0, col: e.1 };
            match Grid::get_subgrid_start(c) {
                Ok(actual) => assert_eq!(actual, expected),
                Err(_) => unreachable!(),
            }
        }
    }
}
