mod board;

use board::Board;

fn main() -> Result<(), board::Error> {
    let mut board = Board::new_empty();
    board.set_cell(8, 8, 9)?;
    board.set_cell(3, 4, 1)?;
    println!("{}", board);

    Ok(())
}
