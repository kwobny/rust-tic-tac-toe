use super::board::{Board, Player};

fn render_player(out: &mut String, player: Option<Player>) {
    out.push(match player {
        None => ' ',
        Some(x) => match x {
            Player::X => 'X',
            Player::O => 'O',
        },
    });
}

fn render_row(out: &mut String, row: &[Option<Player>]) {
    let mut iter = row.iter().peekable();
    if iter.peek().is_none() {
        return;
    }
    render_player(out, *iter.next().unwrap());
    for player in iter {
        out.push('|');
        render_player(out, *player);
    }
}

/// This function returns a string that represents a graphical
/// view of the board. It has newlines in it, and has a newline
/// at the end. Use print!() to display it.
pub fn render_board(board: Board) -> String {
    let contents = board.contents();
    let mut chunks = contents.chunks(board.width());
    let mut out = String::new();

    render_row(&mut out, chunks.next().unwrap());
    for row in chunks {
        for _ in 0..row.len() {
            out.push_str("--");
        }
        render_row(&mut out, row);
    }

    out
}
