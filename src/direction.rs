
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for (i32, i32) {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        }
    }
}

impl Direction {
    pub fn inverted(&self) -> Direction {
        match self {
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Up,
        }
    }
}