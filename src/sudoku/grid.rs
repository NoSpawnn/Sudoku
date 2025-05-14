use super::error::Error;
use std::fmt::{Debug, Display};

use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coordinate {
    pub row: usize,
    pub col: usize,
}

pub struct Grid {
    pub cells: Vec<Cell>,
}

impl Grid {
    pub const SUBGRID_ROWS: usize = 3;
    pub const SUBGRID_COLS: usize = 3;
    pub const SUBGRID_COUNT: usize = 3;
    pub const ROW_COUNT: usize = Self::SUBGRID_ROWS * Self::SUBGRID_COUNT;
    pub const COL_COUNT: usize = Self::SUBGRID_COLS * Self::SUBGRID_COUNT;
    pub const CELL_COUNT: usize = Self::ROW_COUNT * Self::COL_COUNT;
    pub const MIN_CELL_VALUE: u8 = 1;
    pub const MAX_CELL_VALUE: u8 = 9;

    pub fn new_empty() -> Self {
        let cells = (0..Self::ROW_COUNT)
            .flat_map(|row| (0..Self::COL_COUNT).map(move |col| Cell::new(row, col)))
            .collect();
        Self { cells }
    }

    pub fn new_random() -> Self {
        let mut grid = Grid::new_empty();
        grid.solve();
        grid
    }

    pub fn from(data: &[[u8; Self::COL_COUNT]; Self::ROW_COUNT]) -> Self {
        let mut grid = Grid::new_empty();

        for (row_idx, row) in data.iter().enumerate() {
            for (col_idx, value) in row.iter().enumerate() {
                let value = *value;
                if value != 0 {
                    match grid.set_cell(
                        Coordinate {
                            row: row_idx,
                            col: col_idx,
                        },
                        CellState::Filled(value),
                    ) {
                        Ok(_) => continue,
                        Err(e) => panic!("{:?}", e),
                    }
                }
            }
        }

        grid
    }

    pub fn solve(&mut self) {
        const MAX_RECURSE: usize = 20_000_000; // Kinda arbitrary, but an ok safety net

        let nums: Vec<u8> = (Self::MIN_CELL_VALUE..=Self::MAX_CELL_VALUE).collect();
        let mut rng = rand::rng();

        fn fill(
            grid: &mut Grid,
            nums: Vec<u8>,
            rng: &mut impl rand::Rng,
            recurse_counter: &mut usize,
        ) -> bool {
            *recurse_counter += 1;
            if *recurse_counter >= MAX_RECURSE {
                panic!("Failed to generate random grid in {} attempts", MAX_RECURSE);
            }

            let coord = match grid
                .cells
                .iter()
                .find(|c| matches!(c.state, CellState::Empty))
            {
                Some(c) => c.coordinate,
                None => return true,
            };
            let mut shuffled = nums.clone();
            shuffled.shuffle(rng);

            for num in &shuffled {
                let num = *num;
                if matches!(grid.can_place_at(coord, num), Ok(true)) {
                    grid.set_cell_unchecked(coord, CellState::Filled(num));
                    if fill(grid, nums.clone(), rng, recurse_counter) {
                        return true;
                    }
                    grid.set_cell_unchecked(coord, CellState::Empty);
                }
            }

            false
        }

        let mut recurse_counter = 0;
        loop {
            if fill(self, nums.clone(), &mut rng, &mut recurse_counter) {
                break;
            }
        }
    }

    pub fn can_place_in_row(&self, row: usize, value: u8) -> Result<bool, Error> {
        if row >= Self::ROW_COUNT {
            return Err(Error::CellIndexOutOfRange(Coordinate { row: row, col: 0 }));
        } else if value < Self::MIN_CELL_VALUE || value > Self::MAX_CELL_VALUE {
            return Err(Error::ValueOutOfRange(value));
        }

        Ok(self.can_place_in_row_unchecked(row, value))
    }

    fn can_place_in_row_unchecked(&self, row: usize, value: u8) -> bool {
        self.cells[row * Self::COL_COUNT..(row + 1) * Self::COL_COUNT]
            .iter()
            .all(|c| !matches!(c.state, CellState::Filled(v) if v == value))
    }

    pub fn can_place_in_column(&self, col: usize, value: u8) -> Result<bool, Error> {
        if col >= Self::COL_COUNT {
            return Err(Error::CellIndexOutOfRange(Coordinate { row: 0, col: col }));
        } else if value < Self::MIN_CELL_VALUE || value > Self::MAX_CELL_VALUE {
            return Err(Error::ValueOutOfRange(value));
        }

        Ok(self.can_place_in_column_unchecked(col, value))
    }

    fn can_place_in_column_unchecked(&self, col: usize, value: u8) -> bool {
        self.cells
            .iter()
            .skip(col)
            .step_by(Self::COL_COUNT)
            .all(|c| !matches!(c.state, CellState::Filled(v) if v == value))
    }

    pub fn can_place_in_subgrid(&self, c: Coordinate, value: u8) -> Result<bool, Error> {
        if c.row >= Self::ROW_COUNT || c.col >= Self::COL_COUNT {
            return Err(Error::CellIndexOutOfRange(c));
        } else if value < Self::MIN_CELL_VALUE || value > Self::MAX_CELL_VALUE {
            return Err(Error::ValueOutOfRange(value));
        }

        Ok(self.can_place_in_subgrid_unchecked(c, value))
    }

    fn can_place_in_subgrid_unchecked(&self, c: Coordinate, value: u8) -> bool {
        let start = Grid::get_subgrid_start_unchecked(&c);

        for sub_row in start.row..start.row + Self::SUBGRID_ROWS {
            for sub_col in start.col..start.col + Self::SUBGRID_COLS {
                match self
                    .cell_at(Coordinate {
                        row: sub_row,
                        col: sub_col,
                    })
                    .unwrap()
                    .state
                {
                    CellState::Filled(current) if current == value => return false,
                    _ => continue,
                }
            }
        }

        true
    }

    pub fn can_place_at(&self, c: Coordinate, value: u8) -> Result<bool, Error> {
        if c.row >= Self::ROW_COUNT || c.col >= Self::COL_COUNT {
            return Err(Error::CellIndexOutOfRange(c));
        } else if value < Self::MIN_CELL_VALUE || value > Self::MAX_CELL_VALUE {
            return Err(Error::ValueOutOfRange(value));
        }

        Ok(self.can_place_in_column_unchecked(c.col, value)
            && self.can_place_in_row_unchecked(c.row, value)
            && self.can_place_in_subgrid_unchecked(c, value))
    }

    pub fn cell_at(&self, c: Coordinate) -> Result<&Cell, Error> {
        if c.row >= Self::ROW_COUNT || c.col >= Self::COL_COUNT {
            return Err(Error::CellIndexOutOfRange(c));
        }

        Ok(self.cell_at_unchecked(c))
    }

    pub fn cell_at_unchecked(&self, c: Coordinate) -> &Cell {
        &self.cells[c.row * Self::COL_COUNT + c.col]
    }

    pub fn cell_at_mut(&mut self, c: Coordinate) -> Result<&mut Cell, Error> {
        if c.row >= Self::ROW_COUNT || c.col >= Self::COL_COUNT {
            return Err(Error::CellIndexOutOfRange(c));
        }

        Ok(self.cell_at_mut_unchecked(c))
    }

    pub fn cell_at_mut_unchecked(&mut self, c: Coordinate) -> &mut Cell {
        &mut self.cells[c.row * Self::COL_COUNT + c.col]
    }

    pub fn get_subgrid_start(c: &Coordinate) -> Result<Coordinate, Error> {
        if c.row >= Self::ROW_COUNT || c.col >= Self::COL_COUNT {
            return Err(Error::CellIndexOutOfRange(c.clone()));
        }

        Ok(Grid::get_subgrid_start_unchecked(c))
    }

    fn get_subgrid_start_unchecked(c: &Coordinate) -> Coordinate {
        Coordinate {
            row: c.row / Self::SUBGRID_ROWS * Self::SUBGRID_ROWS,
            col: c.col / Self::SUBGRID_COLS * Self::SUBGRID_COLS,
        }
    }

    pub fn set_cell(&mut self, c: Coordinate, state: CellState) -> Result<(), Error> {
        if c.row >= Self::ROW_COUNT || c.col >= Self::COL_COUNT {
            return Err(Error::CellIndexOutOfRange(c));
        } else if let CellState::Filled(value) = state {
            if value < Self::MIN_CELL_VALUE || value > Self::MAX_CELL_VALUE {
                return Err(Error::ValueOutOfRange(value));
            }
        }

        self.set_cell_unchecked(c, state);

        Ok(())
    }

    pub fn set_cell_unchecked(&mut self, c: Coordinate, state: CellState) {
        let idx = c.row * Self::COL_COUNT + c.col;
        self.cells[idx].state = state;
    }

    pub fn set_cell_solver_modifiable(
        &mut self,
        c: Coordinate,
        solver_modifiable: bool,
    ) -> Result<(), Error> {
        let cell = self.cell_at_mut(c)?;
        cell.solver_modifiable = solver_modifiable;
        Ok(())
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

            match cell.state {
                CellState::Filled(value) => write!(f, "[{value}] ")?,
                CellState::Empty => write!(f, "[ ] ")?,
            }
        }

        Ok(())
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new_empty()
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum CellState {
    #[default]
    Empty,
    Filled(u8),
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub coordinate: Coordinate,
    pub state: CellState,
    pub solver_modifiable: bool,
}

impl Cell {
    fn new(row: usize, col: usize) -> Self {
        Self {
            coordinate: Coordinate { row, col },
            state: CellState::default(),
            solver_modifiable: true,
        }
    }
}
