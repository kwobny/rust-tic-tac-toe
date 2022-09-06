use std::fmt::{Display, Formatter, Write, self};

use super::board::{Board, Player};

/// Render a single cell/player.
fn render_player(out: &mut Formatter, player: Option<Player>) -> Result<(), fmt::Error> {
    // Write out ' ', 'X', or 'O' depending on the player.
    out.write_char(match player {
        None => ' ',
        Some(x) => match x {
            Player::X => 'X',
            Player::O => 'O',
        },
    })
}

/// Render a single row.
fn render_row(out: &mut Formatter, row: &[Option<Player>]) -> Result<(), fmt::Error> {
    // If the row is empty, do not render anything.
    if row.len() == 0 {
        return Ok(());
    }

    // Render each cell in the row, inserting boundaries in between.
    write!(out, " ")?;
    render_player(out, row[0])?;
    for player in &row[1..] {
        out.write_char('|')?;
        render_player(out, *player)?;
    }
    out.write_char('\n')?;

    Ok(())
}

impl Display for Board {
    /// Display a board as a string.
    /// This display has newlines in it, including a newline at the end.
    /// To print this, write: print!("{board}");
    fn fmt(&self, mut out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Split the game board into an iterator of rows.
        let contents = self.contents();
        let mut chunks = contents.chunks(self.width());

        // Render each row, placing boundaries between
        // each row.
        render_row(&mut out, chunks.next().unwrap())?;
        for row in chunks {
            // Render the boundary between each row.
            // It should look something like "--+-+--".
            out.write_str("--")?;
            for _ in 1..row.len() {
                out.write_str("+-")?;
            }
            out.write_str("-")?;
            out.write_char('\n')?;

            // Render the row.
            render_row(&mut out, row)?;
        }

        Ok(())
    }
}
