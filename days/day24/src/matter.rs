use crate::coords::Xy;

pub enum Item {
    Blocked,
    Clear,
    Entrance,
    Exit,
}

impl From<(char, bool, bool)> for Item {
    fn from((c, first_row, final_row): (char, bool, bool)) -> Self {
        match c {
            '#' => Self::Blocked,
            '.' if first_row => Self::Entrance,
            '.' if final_row => Self::Exit,
            _ => Self::Clear,
        }
    }
}

pub enum Storm {
    Vrt(usize, Box<dyn Iterator<Item = usize>>),
    Hor(Box<dyn Iterator<Item = usize>>, usize),
}

impl From<(Xy, char, usize, usize)> for Storm {
    fn from((Xy(x, y), c, max_x, max_y): (Xy, char, usize, usize)) -> Self {
        match c {
            '^' => Self::Vrt(x, Box::new((1..max_y).rev().cycle().skip(max_y - y))),
            'v' => Self::Vrt(x, Box::new((1..max_y).cycle().skip(y))),
            '<' => Self::Hor(Box::new((1..max_x).rev().cycle().skip(max_x - x)), y),
            '>' => Self::Hor(Box::new((1..max_x).cycle().skip(x)), y),
            _ => unimplemented!(),
        }
    }
}

impl Storm {
    pub(crate) fn r#move(&mut self) -> Xy {
        match self {
            Storm::Vrt(x, ref mut y, ..) => Xy(*x, y.next().unwrap()),
            Storm::Hor(ref mut x, y, ..) => Xy(x.next().unwrap(), *y),
        }
    }
}
