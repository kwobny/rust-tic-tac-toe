mod board;
mod render;
mod game;

use board::{Board, Player, Coordinates};

pub fn run() -> Result<(), anyhow::Error> {
    game::run(Player::X)?;

    Ok(())
}
