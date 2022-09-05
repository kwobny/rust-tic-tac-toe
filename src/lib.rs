mod board;
mod render;
mod game;

use board::Player;

pub fn run() -> Result<(), anyhow::Error> {
    game::run(Player::X)?;

    Ok(())
}
