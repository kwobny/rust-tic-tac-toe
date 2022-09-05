use std::iter::repeat;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Player {
    X,
    O,
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
#[derive(Clone, Copy, Debug)]
pub struct ValidPosition(Position);

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

    pub fn construct_position(&self, position: Position) -> Result<ValidPosition, ()> {
        for i in [position.x, position.y] {
            if i >= self.width {
                return Err(());
            }
        }
        Ok(ValidPosition(position))
    }

    fn position_to_index(&self, position: ValidPosition) -> usize {
        position.0.y*self.width + position.0.x
    }
    /// Returns Err if the position is already set.
    /// Returns Ok else.
    pub fn set_position(&mut self, player: Player, position: ValidPosition)
        -> Result<&mut Self, ()> {
        let value = self.get_at_position(position);
        if value.is_some() {
            return Err(());
        }
        let index = self.position_to_index(position);
        self.contents[index] = Some(player);

        Ok(self)
    }
    pub fn get_at_position(&self, position: ValidPosition) -> Option<Player> {
        self.contents[self.position_to_index(position)]
    }
    /// Returns the contents of the board in row major order.
    pub fn contents(&self) -> &[Option<Player>] {
        self.contents.as_slice()
    }
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn winner(&self) -> Option<Player> {
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
            if check.is_some() {
                return check;
            }
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
