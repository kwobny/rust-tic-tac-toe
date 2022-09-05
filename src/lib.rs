mod board;
mod render;
mod game;

use board::{Board, Player, Coordinates};

pub fn run() {
    let mut board = Board::new(3).unwrap();

    board.set_position(
        Player::X,
        board.position_from_coordinates(Coordinates {
            x: 0,
            y: 1,
        }).unwrap(),
    ).unwrap();

    board.set_position(
        Player::O,
        board.position_from_coordinates(Coordinates {
            x: 1,
            y: 2,
        }).unwrap(),
    ).unwrap();

    print!("{board}");
}
