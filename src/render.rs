use std::fmt::{Display, Formatter, Write, self};

use super::board::{Board, Player};

fn render_player(out: &mut Formatter, player: Option<Player>) -> Result<(), fmt::Error> {
    out.write_char(match player {
        None => ' ',
        Some(x) => match x {
            Player::X => 'X',
            Player::O => 'O',
        },
    })
}

fn render_row(out: &mut Formatter, row: &[Option<Player>]) -> Result<(), fmt::Error> {
    let mut iter = row.iter().peekable();
    if iter.peek().is_none() {
        return Ok(());
    }
    write!(out, " ")?;
    render_player(out, *iter.next().unwrap())?;
    for player in iter {
        out.write_char('|')?;
        render_player(out, *player)?;
    }
    out.write_char('\n')?;

    Ok(())
}

/// This display has newlines in it, including a newline at the end.
/// Use this with: print!("{board}");
impl Display for Board {
    fn fmt(&self, mut out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let contents = self.contents();
        let mut chunks = contents.chunks(self.width());

        render_row(&mut out, chunks.next().unwrap())?;
        for row in chunks {
            out.write_str("--")?;
            for _ in 1..row.len() {
                out.write_str("+-")?;
            }
            out.write_str("-")?;

            out.write_char('\n')?;
            render_row(&mut out, row)?;
        }

        Ok(())
    }
}
