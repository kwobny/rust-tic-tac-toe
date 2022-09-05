use std::{iter::repeat, fmt::{Display, Write}};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Player {
    X,
    O,
}
impl Player {
    pub fn toggle(self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
    pub fn toggle_in_place(&mut self) {
        *self = self.toggle();
    }
}
impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "player {}",
            match self {
                Player::O => "O",
                Player::X => "X",
            },
        )
    }
}

pub enum WinKind {
    Tie,
    Win(Player),
}

#[derive(Clone, Copy, Debug)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}
/// The stored field is an index into the contents array.
#[derive(Clone, Copy, Debug)]
pub struct ValidPosition(usize);

pub struct Board {
    /// The contents of a board, in row major order.
    contents: Vec<Option<Player>>,
    width: usize,
}
impl Board {
    /// width must not be zero.
    /// This function returns Err if that is the case.
    pub fn new(width: usize) -> Result<Board, ()> {
        if width == 0 {
            return Err(());
        }

        let size = width.pow(2);
        let mut contents = Vec::with_capacity(size);
        contents.extend(repeat(None).take(size));
        Ok(Board {
            contents,
            width,
        })
    }

    /// Returns err if coordinates are out of bounds.
    pub fn position_from_coordinates(&self, position: Coordinates) -> Result<ValidPosition, ()> {
        for i in [position.x, position.y] {
            if i >= self.width {
                return Err(());
            }
        }
        Ok(ValidPosition(position.y*self.width + position.x))
    }
    /// Returns Err if index is out of bounds.
    pub fn position_from_index(&self, index: usize) -> Result<ValidPosition, ()> {
        if index >= self.contents.len() {
            return Err(());
        }
        Ok(ValidPosition(index))
    }

    /// Returns Err if the position is already set.
    /// Returns Ok else.
    pub fn set_position(&mut self, player: Player, position: ValidPosition)
        -> Result<&mut Self, ()> {
        let value = self.get_at_position(position);
        if value.is_some() {
            return Err(());
        }
        self.contents[position.0] = Some(player);

        Ok(self)
    }
    pub fn get_at_position(&self, position: ValidPosition) -> Option<Player> {
        self.contents[position.0]
    }
    /// Returns the contents of the board in row major order.
    pub fn contents(&self) -> &[Option<Player>] {
        self.contents.as_slice()
    }
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn winner(&self) -> Option<WinKind> {
        let mut lines = Vec::new();
        // Horizontal lines.
        for row in (0..self.contents.len()).step_by(self.width) {
            lines.push((row..row+self.width).step_by(1));
        }
        // Vertical lines.
        for column in 0..self.width {
            lines.push((column..self.contents.len()).step_by(self.width));
        }
        // Diagonals.
        lines.push((0..self.contents.len()).step_by(self.width+1));
        lines.push((self.width-1..self.contents.len()).step_by(self.width-1));

        for iter in lines {
            let check = self.check_if_same_player(iter);
            if let Some(win) = check {
                return Some(WinKind::Win(win));
            }
        }

        // Check if is tie.
        let is_tie = self.contents.iter().all(|cell| cell.is_some());
        if is_tie {
            return Some(WinKind::Tie);
        }

        None
    }
    /// Returns None if:
    /// - The indices is empty.
    /// - The player at the index is None.
    /// - The players are not all the same.
    /// Returns Some if else.
    fn check_if_same_player(&self, indices: impl IntoIterator<Item = usize>) -> Option<Player> {
        let mut indices = indices.into_iter().peekable();
        if indices.peek().is_none() {
            return None;
        }
        let test_player = self.contents[indices.next().unwrap()];
        if test_player.is_none() {
            return None;
        }

        for index in indices {
            if self.contents[index] != test_player {
                return None;
            }
        }
        
        Some(test_player.unwrap())
    }
}
