mod grid;

use grid::Grid;

fn main() -> Result<(), grid::Error> {
    let mut grid = Grid::new_empty();
    grid.set_cell(8, 8, 9)?;
    grid.set_cell(3, 4, 1)?;
    println!("{}", grid);

    Ok(())
}
