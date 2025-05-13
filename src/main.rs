mod grid;

use grid::Grid;

fn main() -> Result<(), grid::Error> {
    let mut grid = Grid::new_random();
    println!("{}", grid);

    Ok(())
}
