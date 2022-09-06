use std::{iter, fmt::Display};

/// Represents a player. There are two players,
/// X and O.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Player {
    X,
    O,
}
impl Player {
    // Return the player opposite of this one.
    pub fn toggle(self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
    // Set this player to the opposite of the current player.
    pub fn toggle_in_place(&mut self) {
        *self = self.toggle();
    }
}
impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "{}",
            match self {
                Player::O => "O",
                Player::X => "X",
            },
        )
    }
}

/// An enum representing the end result of a game.
pub enum EndResult {
    /// The game ended in a tie.
    Tie,
    /// The denoted player won the game.
    Win(Player),
    /// Both players occupy winning lines.
    BothWin,
}

/// Represents coordinates on a tic tac toe board.
/// These coordinates are not bounded. To convert
/// coordinates into an actual position usable by
/// one of the functions on the board, convert it
/// to a valid position using the position_from_*
/// methods on the [`Board`] struct.
#[derive(Clone, Copy, Debug)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}
/// An opaque value referring to some position on the game board.
/// This is what is accepted by board functions to refer
/// to positions on the board. It will always be a valid position.
/// Obtain a value of this type using the position_from_*
/// methods on the [`Board`] struct.
// The stored field is an index into the contents array.
#[derive(Clone, Copy, Debug)]
pub struct ValidPosition(usize);

/// A struct that represents a tic tac toe board.
/// There are several functions available for manipulating
/// the board.
pub struct Board {
    // The contents of a board, in row major order.
    contents: Vec<Option<Player>>,
    // The width of the board.
    width: usize,
}
impl Board {
    /// Create a new, empty board with a certain width.
    /// The width must not be zero.
    /// This function returns Err if the width is zero.
    /// It returns Ok in all other cases.
    pub fn new(width: usize) -> Result<Board, ()> {
        if width == 0 {
            return Err(());
        }

        // Create a new vector containing the contents of the board.
        let size = width.pow(2);
        let mut contents = Vec::with_capacity(size);
        contents.extend(iter::repeat(None).take(size));

        Ok(Board {
            contents,
            width,
        })
    }

    /// Get a board position from coordinates.
    /// Returns Err if coordinates are out of bounds.
    pub fn position_from_coordinates(&self, position: Coordinates) -> Result<ValidPosition, ()> {
        // Check if coordinates are within bounds.
        for i in [position.x, position.y] {
            if i >= self.width {
                return Err(());
            }
        }
        Ok(ValidPosition(position.y*self.width + position.x))
    }
    /// Get a board position from an index.
    /// The index should refer to the board in row major order.
    /// Returns Err if index is out of bounds.
    pub fn position_from_index(&self, index: usize) -> Result<ValidPosition, ()> {
        // Check if index is within bounds.
        if index >= self.contents.len() {
            return Err(());
        }
        Ok(ValidPosition(index))
    }

    /// Set the value of a certain cell on the board to the provided player.
    /// Returns Err if the position is already set to something.
    pub fn set_position(&mut self, player: Player, position: ValidPosition)
        -> Result<&mut Self, ()> {
        let value = self.contents[position.0];
        if value.is_some() {
            return Err(());
        }
        self.contents[position.0] = Some(player);

        Ok(self)
    }
    /// Get the value of a certain cell.
    /// Returns None if the cell is not set.
    /// Returns Some if the cell is set to a certain player.
    pub fn get_at_position(&self, position: ValidPosition) -> Option<Player> {
        self.contents[position.0]
    }
    /// Returns the contents of the board in row major order.
    pub fn contents(&self) -> &[Option<Player>] {
        self.contents.as_slice()
    }
    /// Returns the width of the board.
    pub fn width(&self) -> usize {
        self.width
    }

    /// This function tests whether the game has ended and
    /// returns the result of the game.
    /// This function is designed so that it can be called anytime,
    /// even if the game has technically ended but the code still
    /// continues to execute. It accounts for the possibility of both
    /// players occupying win lines, which probably indicates a bug.
    /// It returns None if the game has not ended.
    /// It returns Some with the end result, if the game has ended.
    pub fn potential_end_result(&self) -> Option<EndResult> {
        // Create a list of possible lines to check for a win.

        // Horizontal lines.
        let horizontal = (0..self.contents.len()).step_by(self.width).map(|row| {
            (row..row+self.width).step_by(1)
        });
        // Vertical lines.
        let vertical = (0..self.width).map(|column| {
            (column..self.contents.len()).step_by(self.width)
        });
        // Diagonals.
        let backward_diagonal = (0..self.contents.len()).step_by(self.width+1);
        let forward_diagonal = (self.width-1..self.contents.len()).step_by(self.width-1);

        let win_lines =
            (horizontal)
            .chain(vertical)
            .chain(iter::once(forward_diagonal))
            .chain(iter::once(backward_diagonal));

        // For each win line, check if all cells on that line
        // are set to the same player. If so, that means that
        // player has won. If both players occupy win lines,
        // then both players have "won".
        let mut player_won = None;
        for line in win_lines {
            let check = self.check_if_same_player(line);
            if check.is_some() {
                if player_won.is_none() {
                    player_won = check;
                } else if player_won.unwrap() != check.unwrap() {
                    return Some(EndResult::BothWin);
                }
            }
        }
        if let Some(win) = player_won {
            return Some(EndResult::Win(win));
        }

        // Check if all cells on the board have been set.
        // If so, then that means it's a tie.
        let is_tie = self.contents.iter().all(|cell| cell.is_some());
        if is_tie {
            return Some(EndResult::Tie);
        }

        // The game has not ended.
        None
    }
    /// Check if the cells indicated by the provided iterator
    /// are all set to the same player.
    /// Returns None if:
    /// - The indices iterator is empty.
    /// - Any of the board cells indicated by the iterator
    ///   contain None (are unoccupied).
    /// - The players are not all the same.
    /// Returns Some if else.
    fn check_if_same_player(&self, indices: impl IntoIterator<Item = usize>) -> Option<Player> {
        let mut indices = indices.into_iter().peekable();
        // Check if the indices iterator is empty.
        if indices.peek().is_none() {
            return None;
        }

        // Get the first cell. This will be compared to all
        // subsequent cells. Check if the first cell is None.
        let test_player = self.contents[indices.next().unwrap()];
        if test_player.is_none() {
            return None;
        }

        // Check if any of the cells are not equal to the first cell.
        for index in indices {
            if self.contents[index] != test_player {
                return None;
            }
        }
        
        Some(test_player.unwrap())
    }
}
