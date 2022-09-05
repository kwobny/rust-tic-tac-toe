mod board;
mod render;
mod game;

use board::{Board, Player, Coordinates};

pub fn run() -> Result<(), anyhow::Error> {
    let board = Board::new(3).unwrap();

    game::run(board, Player::X)?;

    Ok(())
}
