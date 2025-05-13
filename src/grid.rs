use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum Error {
    CellIndexOutOfRange(Coordinate),
    ValueOutOfRange(u8),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Coordinate {
    pub row: usize,
    pub col: usize,
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

    pub fn can_place_in_row(&self, row: usize, value: u8) -> Result<bool, Error> {
        if row >= Self::ROW_COUNT {
            return Err(Error::CellIndexOutOfRange(Coordinate { row: row, col: 0 }));
        } else if value <= Self::MIN_CELL_VALUE || value > Self::MAX_CELL_VALUE {
            return Err(Error::ValueOutOfRange(value));
        }

        Ok(
            self.cells[row * Self::COL_COUNT..(row + 1) * Self::COL_COUNT]
                .iter()
                .all(|c| !matches!(c, Cell::Filled(v) if *v == value)),
        )
    }

    pub fn can_place_in_column(&self, col: usize, value: u8) -> Result<bool, Error> {
        if col >= Self::COL_COUNT {
            return Err(Error::CellIndexOutOfRange(Coordinate { row: 0, col: col }));
        } else if value <= Self::MIN_CELL_VALUE || value > Self::MAX_CELL_VALUE {
            return Err(Error::ValueOutOfRange(value));
        }

        Ok(self
            .cells
            .iter()
            .skip(col)
            .step_by(Self::COL_COUNT)
            .all(|c| !matches!(c, Cell::Filled(v) if *v == value)))
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

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
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

    #[test]
    fn test_can_place_in_row() {
        let mut grid = Grid::new_empty();
        let _ = grid.set_cell(Coordinate { row: 0, col: 0 }, 1);
        assert!(!grid.can_place_in_row(0, 1).unwrap());
        let _ = grid.set_cell(Coordinate { row: 0, col: 8 }, 9);
        assert!(!grid.can_place_in_row(0, 9).unwrap());
        assert!(grid.can_place_in_row(0, 8).unwrap());
    }
    #[test]
    fn test_can_place_in_column() {
        let mut grid = Grid::new_empty();
        let _ = grid.set_cell(Coordinate { row: 0, col: 0 }, 1);
        assert!(!grid.can_place_in_column(0, 1).unwrap());
        let _ = grid.set_cell(Coordinate { row: 6, col: 0 }, 9);
        assert!(!grid.can_place_in_column(0, 9).unwrap());
        assert!(grid.can_place_in_column(0, 8).unwrap());
    }
}
