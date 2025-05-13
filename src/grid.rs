use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum Error {
    CellIndexOutOfRange { row: usize, col: usize },
    ValueOutOfRange(u8),
}

pub struct Grid {
    pub cells: Vec<Cell>,
}

impl Grid {
    const ROW_COUNT: usize = 9;
    const COL_COUNT: usize = 9;
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

    pub fn set_cell(&mut self, row: usize, col: usize, value: u8) -> Result<(), Error> {
        if row >= Self::ROW_COUNT || col >= Self::COL_COUNT {
            return Err(Error::CellIndexOutOfRange { row, col });
        } else if value <= Self::MIN_CELL_VALUE || value > Self::MAX_CELL_VALUE {
            return Err(Error::ValueOutOfRange(value));
        }

        self.set_cell_unchecked(row, col, value);

        Ok(())
    }

    fn set_cell_unchecked(&mut self, row: usize, col: usize, value: u8) {
        let idx = row * Self::COL_COUNT + col;
        self.cells[idx] = Cell::Filled(value);
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        write!(f, " ")?;
        for i in 1..Self::COL_COUNT + 1 {
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
