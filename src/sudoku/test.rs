#[cfg(test)]
use super::grid::*;

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
        match Grid::get_subgrid_start(&c) {
            Ok(actual) => assert_eq!(actual, expected),
            Err(_) => unreachable!(),
        }
    }
}

#[test]
fn test_can_place_in_row() {
    let mut grid = Grid::new_empty();
    let _ = grid.set_cell(Coordinate { row: 0, col: 0 }, CellState::Filled(1));
    assert!(!grid.can_place_in_row(0, 1).unwrap());
    let _ = grid.set_cell(Coordinate { row: 0, col: 8 }, CellState::Filled(9));
    assert!(!grid.can_place_in_row(0, 9).unwrap());
    assert!(grid.can_place_in_row(0, 8).unwrap());
}

#[test]
fn test_can_place_in_column() {
    let mut grid = Grid::new_empty();
    let _ = grid.set_cell(Coordinate { row: 0, col: 0 }, CellState::Filled(1));
    assert!(!grid.can_place_in_column(0, 1).unwrap());
    let _ = grid.set_cell(Coordinate { row: 6, col: 0 }, CellState::Filled(9));
    assert!(!grid.can_place_in_column(0, 9).unwrap());
    assert!(grid.can_place_in_column(0, 8).unwrap());
}

#[test]
fn test_can_place_in_subgrid() {
    let mut grid = Grid::new_empty();
    let _ = grid.set_cell(Coordinate { row: 0, col: 0 }, CellState::Filled(1));
    assert!(
        !grid
            .can_place_in_subgrid(Coordinate { row: 2, col: 2 }, 1)
            .unwrap()
    );
}
