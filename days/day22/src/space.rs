#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Xy(pub usize, pub usize);

pub enum Tile {
    Open,
    Wall,
}

impl Tile {
    pub fn is_open(&self) -> bool {
        matches!(self, Self::Open)
    }
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Open),
            '#' => Ok(Self::Wall),
            _ => Err(()),
        }
    }
}

pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn turn(&self, rotation: &Self) -> Self {
        match self {
            Dir::Up => match rotation {
                Dir::Left => Self::Left,
                Dir::Right => Self::Right,
                _ => unimplemented!(),
            },
            Dir::Down => match rotation {
                Dir::Left => Self::Right,
                Dir::Right => Self::Left,
                _ => unimplemented!(),
            },
            Dir::Left => match rotation {
                Dir::Left => Self::Down,
                Dir::Right => Self::Up,
                _ => unimplemented!(),
            },
            Dir::Right => match rotation {
                Dir::Left => Self::Up,
                Dir::Right => Self::Down,
                _ => unimplemented!(),
            },
        }
    }
}

pub enum Step {
    Fwd(usize),
    Trn(Dir),
}
