#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Xy(pub usize, pub usize);

impl Xy {
    pub fn new(x: usize, y: usize) -> Self {
        Xy(x, y)
    }

    pub fn x(&self) -> &usize {
        &self.0
    }

    pub fn y(&self) -> &usize {
        &self.1
    }

    pub fn adj(&self, dir: Dir) -> Xy {
        let Xy(x, y) = self;
        match dir {
            Dir::Up => Xy(*x, y - 1),
            Dir::Down => Xy(*x, y + 1),
            Dir::Left => Xy(x - 1, *y),
            Dir::Right => Xy(x + 1, *y),
        }
    }
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub enum Dir {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
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

#[derive(Debug)]
pub enum Step {
    Fwd(isize),
    Trn(Dir),
}
